#[cfg(feature = "server")]
use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn start(app_fn: fn() -> Element) {
    //
    use crate::{auth::*, server::connect_to_pbdb};

    use axum::routing::*;
    use axum_session::{SessionConfig, SessionPgPool, SessionStore};
    use axum_session_auth::AuthConfig;
    use dioxus::prelude::*;

    init_logging();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            // Skipping this, for the sake of this example.
            // log::info!("Connecting to the database ...");
            // let pg_pool = connect_to_pbdb().await;
            // if pg_pool.is_err() {
            //     eprintln!("Failed to connect to database! Exiting now.");
            //     return;
            // }
            // let pg_pool = pg_pool.unwrap();
            // log::info!("Connected to the database.");

            // This defaults as normal cookies.
            // let session_config = SessionConfig::default().with_table_name("users_sessions");
            // let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
            // let session_store =
            //     SessionStore::<SessionPgPool>::new(Some(pg_pool.clone().into()), session_config)
            //         .await
            //         .unwrap();

            // Skipping this, for the sake of this example.
            // User::create_user_tables(&pg_pool).await;

            // Build our application web api router.
            let web_api_router = Router::new()
                // Server side render the application, serve static assets, and register server functions.
                .serve_dioxus_application(ServeConfig::builder().build(), move || {
                    VirtualDom::new(app_fn)
                })
                // Skipping this, for the sake of this example.
                // .await
                // .layer(
                //     axum_session_auth::AuthSessionLayer::<
                //         crate::auth::User,
                //         i64,
                //         axum_session_auth::SessionPgPool,
                //         sqlx::PgPool,
                //     >::new(Some(pg_pool))
                //     .with_config(auth_config),
                // )
                // .layer(axum_session::SessionLayer::new(session_store));
                //
                // And just complete the statement.
                .await;

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
        .with_module_level("dioxus_core", LevelFilter::Warn)
        .with_module_level("dioxus_signals", LevelFilter::Info)
        .with_module_level("tracing", LevelFilter::Warn)
        .init()
        .unwrap();
}
