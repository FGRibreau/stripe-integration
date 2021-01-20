use config::{self, Environment, File};
use serde::Deserialize;

use crate::errors::AppError;

#[derive(Clone, Deserialize)]
pub struct Config {
    // App ip and port
    pub port: String,
    pub ip: String,

    // Stripe API
    pub stripe_api_secret_key: String,
    pub stripe_api_public_key: String,

    // Stripe Checkout
    pub stripe_allow_promotion_codes: bool,
    pub stripe_checkout_cancel_url: String,
    pub stripe_checkout_success_url: String,

    // Sentry
    pub sentry_dsn: String,
}

impl Config {
    pub fn load(path: Option<String>) -> Result<Config, AppError> {
        let mut s = config::Config::new();
        //s.merge(File::with_name("config/default"))?;

        if let Some(p) = path {
            s.merge(File::with_name(&p))?;
        }

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::new())?;
        s.try_into().map_err(|e| e.into())
    }
}
