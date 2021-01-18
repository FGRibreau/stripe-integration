use actix_web::web::Data;
use actix_web::{http, web, Error, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde::Serialize;

use crate::api::ApiState;
use crate::stripe::{CheckoutSessionMode, CheckoutSessionParam, LineItem, PaymentMethodType};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutParams {
    publishable_key: String,
    session_id: String,
}

pub async fn display(params: web::Query<CheckoutParams>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-type", "text/html")
        .body(format!(
            "
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8' />
    <title>Checkout</title>
    <script src='https://js.stripe.com/v3/'></script>
  </head>
  <body>
   <script>
    var stripe = Stripe(\"{publishable_key}\");
    stripe.redirectToCheckout({{sessionId: \"{session_id}\"}}).then(function(result){{
        // @todo improve this
        if(result.error){{
            alert(result.error);
            console.error(result.error);
        }}
     }});
   </script>
  </body>
</html>
",
            publishable_key = params.publishable_key,
            session_id = params.session_id
        )))
}

pub async fn go(api_state: Data<ApiState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let price_id = req
        .match_info()
        .get("price_id")
        .ok_or_else(|| HttpResponse::NotFound().body("Plan ID required."))?;

    let session = api_state.stripe_client.create_checkout_session(CheckoutSessionParam {
        mode: CheckoutSessionMode::Subscription,
        success_url: api_state.configuration.stripe_checkout_success_url.clone(),
        cancel_url: api_state.configuration.stripe_checkout_cancel_url.clone(),
        payment_method_types: vec![PaymentMethodType::Card],
        line_items: vec![LineItem {
            price: price_id.to_string(),
            quantity: 1,
        }],
        ..CheckoutSessionParam::default()
    }).await.map_err(|x| {
        error!("{:?}", x);
        HttpResponse::InternalServerError().body("We could not contact our payment provider. Please try again or contact our support team.")
    })?;

    let query_params = serde_qs::to_string(&CheckoutParams {
        publishable_key: api_state.configuration.stripe_api_public_key.clone(),
        session_id: session.id,
    })
    .map_err(|x| {
        error!("{:?}", x);
        HttpResponse::InternalServerError().body("An error occurred, please try again later.")
    })?;

    let url = req
        .url_for_static("checkout_display")
        .expect("could not reverse find the checkout url");
    Ok(HttpResponse::TemporaryRedirect()
        .header(http::header::LOCATION, format!("{}?{}", url, query_params))
        .finish())
}
