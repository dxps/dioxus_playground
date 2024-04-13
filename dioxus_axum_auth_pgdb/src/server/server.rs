use dioxus::prelude::*;

#[cfg(feature = "server")]
pub fn server_start(app_fn: fn() -> Element) {
    use crate::auth::*;
    use crate::ui::app;

    use axum::routing::*;
    use axum_session::SessionConfig;
    use axum_session::SessionPgPool;
    use axum_session::SessionStore;
    use axum_session_auth::AuthConfig;

    use dioxus_fullstack::prelude::*;

    simple_logger::SimpleLogger::new().init().unwrap();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            let pg_pool = connect_to_pbdb().await;
            if pg_pool.is_err() {
                eprintln!("Failed to connect to database! Exiting now.");
                return;
            }
            let pg_pool = pg_pool.unwrap();
            println!("Connected to the database.");
            dbg!(&pg_pool);

            //This Defaults as normal Cookies.
            //To enable Private cookies for integrity, and authenticity please check the next Example.
            let session_config = SessionConfig::default().with_table_name("users_sessions");
            let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
            let session_store =
                SessionStore::<SessionPgPool>::new(Some(pg_pool.clone().into()), session_config)
                    .await
                    .unwrap();

            User::create_user_tables(&pg_pool).await;

            // build our application with some routes
            let app = Router::new()
                // Server side render the application, serve static assets, and register server functions
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
                    >::new(Some(pg_pool))
                    .with_config(auth_config),
                )
                .layer(axum_session::SessionLayer::new(session_store));

            // run it
            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        });
}
