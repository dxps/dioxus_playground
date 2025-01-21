use std::str::FromStr;
use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    //
    #[error("{0} already exists")]
    AlreadyExists(String),

    /// Commonly used to indicate that an item deletion cannot be done since
    /// it is referred (mainly at the database level through a foreign key).
    #[error("dependencies exist")]
    DependenciesExist,

    #[error("")]
    Ignorable,

    #[error("internal error")]
    InternalErr,

    /// Generic error.
    #[error("{0}")]
    Err(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("The pair of name and description must be unique.")]
    NameDescriptionNotUnique,
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::Err(s.to_string())
    }
}

impl FromStr for AppError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <AppError as FromStr>::Err> {
        Ok(Self::from(s))
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::from(err.to_string())
    }
}
