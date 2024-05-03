#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState(
    /// The database connection pool.
    pub std::sync::Arc<sqlx::Pool<sqlx::Postgres>>,
);
