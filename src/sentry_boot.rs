#[macro_export]
macro_rules! sentry_boot {
  ($x: expr) => {{
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters("info");
    let log_integration = sentry_log::LogIntegration::default().with_env_logger_dest(Some(log_builder.build()));
    let client = sentry::init((
      $x,
      sentry::ClientOptions {
        send_default_pii: true,
        attach_stacktrace: true,
        ..Default::default()
      }
      .add_integration(log_integration),
    ));

    if client.is_enabled() {
      std::env::set_var("RUST_BACKTRACE", "1");
      log::info!("Sentry integration initialized");
    } else {
      log::warn!("Could not initialize Sentry integration");
    }
    client
  }};
}
