use std::{collections::HashMap, ops::Deref, sync::Mutex, time::Duration};

use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

// Display the book using the format "{title} by {author}".
// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

pub static MEMDB: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Book {
                id: 1,
                title: "Antigone".into(),
                author: "Sophocles".into(),
            },
        ),
        (
            2,
            Book {
                id: 2,
                title: "Beloved".into(),
                author: "Toni Morrison".into(),
            },
        ),
        (
            3,
            Book {
                id: 3,
                title: "Candide".into(),
                author: "Voltaire".into(),
            },
        ),
    ]))
});

pub async fn init_db_pool() -> Result<PgPool, sqlx::Error> {
    log::info!("Initing the DB Pool ...");
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .idle_timeout(Duration::from_secs(280))
        .connect("postgres://tmc:tmc@localhost:5442/tmc")
        .await?;
    Ok(pool)
}

pub static DB: Lazy<PgPool> = Lazy::new(|| {
    log::info!("Initing the DB Pool lazily ...");
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .idle_timeout(Duration::from_secs(280))
        .connect_lazy("postgres://tmc:tmc@localhost:5442/tmc");
    match pool {
        Ok(pool) => {
            log::info!("DB Pool lazily inited.");
            pool
        }
        Err(err) => {
            log::error!("Unable to init the DB Pool: {}", err);
            panic!("Unable to init the DB Pool: {}", err)
        }
    }
});

pub async fn is_db_pool_ready() -> Result<(), sqlx::Error> {
    log::info!("DB Pool idle={} size={}", DB.num_idle(), DB.size());
    let db_pool: &PgPool = DB.deref();
    let row = sqlx::query("SELECT 1").execute(db_pool).await?;

    log::debug!(
        "[is_db_pool_ready] Test rows affected {}",
        row.rows_affected()
    );
    Ok(())
}
