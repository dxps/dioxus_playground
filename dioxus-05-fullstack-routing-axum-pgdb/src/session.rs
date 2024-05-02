use axum::async_trait;

use crate::server::ServerState;

pub struct Session {
    pub dbp: std::sync::Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for Session
where
    S: std::marker::Sync + std::marker::Send,
{
    type Rejection = StateError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        //
        let ss = parts.extensions.get::<ServerState>().unwrap();
        let dbp = ss.0.clone();
        Ok(Session { dbp })
    }
}

#[derive(Debug)]
pub struct StateError;

impl std::error::Error for StateError {}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(internal) state error")
    }
}

impl axum::response::IntoResponse for StateError {
    fn into_response(self) -> axum::response::Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "(internal) state error",
        )
            .into_response()
    }
}
