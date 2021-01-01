use std::sync::Arc;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::dev::Server;
use log::info;

use crate::{config::Config, handlers};
use crate::errors::AppError;
use crate::stripe::StripeClient;

pub struct ApiState {
    pub configuration: Arc<Config>,
    pub stripe_client: StripeClient,
}

pub fn connect(configuration: Config) -> Result<Server, AppError> {
    // Get the port the server must listen on
    let bind_address = format!("{}:{}", configuration.ip, configuration.port);

    let c = Arc::new(configuration);
    // Create the web server
    let server = HttpServer::new(move || {
        let _config = c.clone();
        App::new()
            .wrap(middleware::NormalizePath::default())
            .data(ApiState {
                configuration: _config,
                stripe_client: StripeClient::new(c.stripe_api_secret_key.clone()),
            })
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1.0")
                    .service(
                        web::scope("/checkout")
                            // e.g. http://127.0.0.1:8080/v1.0/checkout/go/enterprise-2016
                            .route(
                                "/go/{price_id}",
                                web::get().to(handlers::checkout::go),
                            )
                            .service(web::resource("/")
                                .name("checkout_display")
                                .route(web::get().to(handlers::checkout::display))
                            )
                    ),
            )
    })
        .bind(&bind_address)?
        .run();

    info!("Started HTTP API on {}", &bind_address);

    Ok(server)
}
