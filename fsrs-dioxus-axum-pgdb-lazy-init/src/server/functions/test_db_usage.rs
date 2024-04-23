// use crate::server::database::DB;
// use dioxus::prelude::*;
// use sqlx::PgPool;
// use std::ops::Deref;

// Commented here since it could not be used in a Dioxus component due to the error:
// 'server module not found in crate'. For later investigation.

// #[server(TestDbUsage)]
// pub async fn test_db_usage() -> Result<String, ServerFnError> {
//     let db_pool: &PgPool = DB.deref();
//     let rs = sqlx::query("SELECT 1")
//         .execute(db_pool)
//         .await
//         .map_err(|err| err.to_string());
//     match rs {
//         Ok(rs) => {
//             let msg = format!("{} rows affected.", rs.rows_affected());
//             log::debug!("{}", msg);
//             Ok(msg)
//         }
//         Err(err) => {
//             let msg = format!("Failed to use db due to '{}'.", err);
//             log::debug!("{}", msg);
//             Err(ServerFnError::Response(msg))
//         }
//     }
// }
