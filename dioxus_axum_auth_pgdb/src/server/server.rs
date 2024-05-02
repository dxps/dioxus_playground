use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn server_start(app_fn: fn() -> Element) {
    use crate::{auth::*, server::ServerState};
    use axum::{routing::*, Extension};
    use axum_session::{SessionConfig, SessionPgPool, SessionStore};
    use axum_session_auth::AuthConfig;
    use dioxus::prelude::*;
    use log::{debug, error};
    use std::sync::Arc;

    init_logging();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            debug!("Connecting to the database ...");
            let pg_pool = connect_to_db().await;
            if pg_pool.is_err() {
                error!("Failed to connect to the database! Exiting now.");
                return;
            }
            let pg_pool = pg_pool.unwrap();
            debug!("Connected to the database.");
            debug!("pg_pool={:?}", &pg_pool);

            // This defaults as normal cookies.
            let session_config = SessionConfig::default().with_table_name("users_sessions");
            let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
            let session_store =
                SessionStore::<SessionPgPool>::new(Some(pg_pool.clone().into()), session_config)
                    .await
                    .unwrap();

            User::create_user_tables(&pg_pool).await;

            let state = ServerState(Arc::new(pg_pool.clone()));

            // Build our application web api router.
            let web_api_router = Router::new()
                // Server side render the application, serve static assets, and register server functions.
                .serve_dioxus_application(ServeConfig::builder().build(), move || {
                    VirtualDom::new(app_fn)
                })
                .await
                .layer(
                    axum_session_auth::AuthSessionLayer::<
                        crate::auth::User,
                        i64,
                        axum_session_auth::SessionPgPool,
                        sqlx::PgPool,
                    >::new(Some(pg_pool.clone()))
                    .with_config(auth_config),
                )
                .layer(axum_session::SessionLayer::new(session_store))
                .layer(Extension(state));

            // Start it.
            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            axum::serve(listener, web_api_router.into_make_service())
                .await
                .unwrap();
        });
}

#[cfg(feature = "server")]
fn init_logging() {
    use log::LevelFilter;

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", LevelFilter::Info)
        .with_module_level("tungstenite", LevelFilter::Info)
        .with_module_level("tokio_tungstenite", LevelFilter::Info)
        .with_module_level("axum_session", LevelFilter::Info)
        .with_module_level("axum_session_auth", LevelFilter::Error)
        .with_module_level("dioxus_core", LevelFilter::Info)
        .with_module_level("dioxus_signals", LevelFilter::Info)
        .with_module_level("tracing", LevelFilter::Warn)
        .init()
        .unwrap();
}
