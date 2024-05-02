#[cfg(feature = "server")]
use sqlx::{postgres::PgPoolOptions, PgPool};

#[cfg(feature = "server")]
pub async fn connect_to_db() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgres://test:test@localhost:5443/test")
        .await?;
    Ok(pool)
}
