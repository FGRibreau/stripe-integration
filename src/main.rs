#[macro_use]
extern crate log;

use crate::errors::AppError;

mod handlers;
mod config;
mod api;
mod sentry_boot;
mod errors;
mod stripe;

/// Entry point
#[actix_rt::main]
async fn main() -> Result<(), AppError> {
    let mut args = std::env::args();
    let _ = args.next();

    let configuration = config::Config::load(args.next())?;

    // Initialise the logger and set the default level to INFO
    let _guard = sentry_boot! { configuration.sentry_dsn.as_str() };

    let server = api::connect(configuration)?;

    server.await.map_err(|err| err.into())
}
