#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState(pub std::sync::Arc<sqlx::Pool<sqlx::Postgres>>);
