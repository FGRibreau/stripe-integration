use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::AppError;

pub struct StripeClient {
    pub secret_key: String,
    pub endpoint: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct CheckoutSessionParam {
    pub mode: CheckoutSessionMode,
    pub success_url: String,
    pub cancel_url: String,
    pub payment_method_types: Vec<PaymentMethodType>,
    pub line_items: Vec<LineItem>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutSession {
    pub id: String,

    pub amount_subtotal: u32,
    pub amount_total: u32,
    pub cancel_url: String,
    pub success_url: String,
    pub currency: String,
    pub livemode: bool,
    pub mode: CheckoutSessionMode,
    pub payment_status: PaymentStatus,

}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Unpaid,
    Paid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethodType {
    Card
}

#[derive(Debug, Serialize, Default)]
pub struct LineItem {
    // The ID of the Price or Plan
    pub price: String,

    // Amount
    pub quantity: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeErrorResponse {
    pub error: StripeError,
}

impl From<StripeErrorResponse> for AppError {
    fn from(err: StripeErrorResponse) -> Self {
        AppError::InvalidStripeRequest {
            code: err.error.code.unwrap_or_default(),
            doc_url: err.error.doc_url.unwrap_or_default(),
            message: err.error.message.unwrap_or_default(),
            param: err.error.param.unwrap_or_default(),
            type_: err.error.type_.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeError {
    pub code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,

    #[serde(rename = "type")]
    pub type_: Option<String>,
}


impl StripeClient {
    pub fn new(api_key: String) -> Self {
        StripeClient {
            secret_key: api_key,
            endpoint: "https://api.stripe.com/v1".into(),
            version: "2020-08-27".into(),
        }
    }
    pub async fn create_checkout_session(&self, params: CheckoutSessionParam) -> Result<CheckoutSession, AppError> {
        let client = reqwest::Client::new();

        match client.post(format!("{endpoint}/checkout/sessions", endpoint = self.endpoint).as_str())
            .basic_auth::<String, String>(self.secret_key.clone(), None)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Stripe-Version", self.version.clone())
            // reqwest.form relies on "serde_urlencoded::to_string" that itself cannot serialize Vec<T>
            // see: https://github.com/nox/serde_urlencoded/issues/46
            // thus we rely on serde_qs to do the job
            .body(serde_qs::to_string(&params)?)
            .send()
            .await {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        response.json::<CheckoutSession>().await.map_err(|x| x.into())
                    }
                    StatusCode::BAD_REQUEST => {
                        Err(response.json::<StripeErrorResponse>().await?.into())
                    }
                    _ => {
                        Err(AppError::UnsupportedStripeResponse { json: response.json::<Value>().await?.clone() })
                    }
                }
            }
            Err(error) => Err(error.into())
        }
    }
}


#[cfg(test)]
#[allow(unused_imports)]
mod unit_tests {
    use crate::stripe::*;

    #[test]
    fn stripe_create_checkout_session_serialize() {
        insta::assert_display_snapshot!(serde_qs::to_string(&CheckoutSessionParam {
            mode: CheckoutSessionMode::Payment,
            success_url: "http://localhost:8080/success_url".to_string(),
            cancel_url: "http://localhost:8080/cancel_url".to_string(),
            payment_method_types: vec![PaymentMethodType::Card],
            line_items: vec![LineItem { price: "enterprise-2016".to_string(), quantity: 1 }],
        }).unwrap(), @"mode=payment&success_url=http%3A%2F%2Flocalhost%3A8080%2Fsuccess_url&cancel_url=http%3A%2F%2Flocalhost%3A8080%2Fcancel_url&payment_method_types[0]=card&line_items[0][price]=enterprise-2016&line_items[0][quantity]=1")
    }
}

#[allow(unused_imports)]
mod integration_tests {
    use crate::config::Config;
    use crate::stripe::{CheckoutSessionMode, CheckoutSessionParam, LineItem, PaymentMethodType, StripeClient};

    #[actix_rt::test]
    async fn stripe_create_checkout_session() {
        let config = Config::load(None).unwrap();

        let client = StripeClient::new(config.stripe_api_secret_key);

        match client.create_checkout_session(CheckoutSessionParam {
            mode: CheckoutSessionMode::Subscription,
            success_url: "http://localhost:8080/success_url".to_string(),
            cancel_url: "http://localhost:8080/cancel_url".to_string(),
            payment_method_types: vec![PaymentMethodType::Card],
            line_items: vec![LineItem { price: env!("TEST_VALID_PRICE_ID").to_string(), quantity: 1 }],
        }).await {
            Ok(session) => {
                //println!("{:?}", session);
                assert_eq!(session.id.is_empty(), false)
            }
            Err(err) => panic!("{:#?}", err)
        }
    }
}
