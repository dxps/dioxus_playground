#[cfg(feature = "server")]
pub fn init_logging() {
    use log::LevelFilter;

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", LevelFilter::Info)
        .with_module_level("tungstenite", LevelFilter::Info)
        .with_module_level("tokio_tungstenite", LevelFilter::Info)
        .with_module_level("axum_session", LevelFilter::Info)
        .with_module_level("axum_session_auth", LevelFilter::Error)
        .with_module_level("dioxus_core", LevelFilter::Warn)
        .with_module_level("dioxus_signals", LevelFilter::Info)
        .with_module_level("tracing", LevelFilter::Warn)
        .init()
        .unwrap();

    log::debug!("Logging setup done.");
}
