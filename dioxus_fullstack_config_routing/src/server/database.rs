use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[cfg(feature = "server")]
pub async fn connect_to_pbdb() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgres://test:test@localhost:5500/test")
        .await?;
    Ok(pool)
}
