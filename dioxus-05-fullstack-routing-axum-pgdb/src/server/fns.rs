use dioxus::prelude::*;

#[server(PostServerData)]
pub async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    //
    use sqlx::{postgres::PgRow, Row};
    use tracing::debug;

    // Interacting with the database.
    let session: crate::session::Session = extract().await?;
    let dbp = session.dbp;
    sqlx::query("SELECT version() as version;")
        .map(|r: PgRow| {
            debug!("Database version: {}", r.get::<String, _>("version"));
        })
        .fetch_all(dbp.as_ref())
        .await?;

    Ok("Hello from the server!".to_string())
}
