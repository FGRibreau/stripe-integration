use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("Stripe API request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON Serialization/Deserialization error: {0}")]
    JSONSerdeError(#[from] serde_json::Error),

    #[error("UrlEncoded Serialization error: {0}")]
    URLEncodedSerializationError(#[from] serde_qs::Error),

    #[error("Unsupported Stripe response: {json:#?}")]
    UnsupportedStripeResponse {
        json: Value
    },

    #[error("invalid stripe request {code:?}: {message:?} (url: {doc_url:?}")]
    InvalidStripeRequest {
        code: String,
        doc_url: String,
        message: String,
        param: String,
        type_: String,
    },
}
