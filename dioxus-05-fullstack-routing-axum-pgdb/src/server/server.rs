use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn server_start(app_fn: fn() -> Element) {
    use axum::{routing::*, Extension};
    use dioxus::prelude::*;
    use std::sync::Arc;
    use tracing::{debug, error, info};

    use crate::server::{db::connect_to_db, ServerState};

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
            info!("Connected to the database.");
            debug!("pg_pool={:?}", &pg_pool);

            let state = ServerState(Arc::new(pg_pool));

            // Build our application web api router.
            let web_api_router = Router::new()
                // Server side render the application, serve static assets, and register server functions.
                .serve_dioxus_application(ServeConfig::builder().build(), move || {
                    VirtualDom::new(app_fn)
                })
                .await
                .layer(Extension(state));

            // Start it.
            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            axum::serve(listener, web_api_router.into_make_service())
                .await
                .unwrap();
        });
}
