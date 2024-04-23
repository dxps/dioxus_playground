#![allow(non_snake_case)]

#[cfg(feature = "server")]
mod server;

use server::database::DB;
// use crate::server::functions::test_db_usage;
use sqlx::PgPool;
use std::ops::Deref;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    #[cfg(feature = "server")]
    server::start(App);

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        br {}
        button {
            onclick: move |_| async move {
                if let Ok(data) = test_db_usage().await {
                    log::info!("Client received: {}", data);
                    text.set(data.clone());
                    post_server_data(data).await.unwrap();
                }
            },
            "Test DB Usage"
        }
        {}
        p { "Server (test_db_usage) data: {text}"}
        {}
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        log::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

// ------------------------------------------------------------------
//                          Server Functions
// ------------------------------------------------------------------

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    log::debug!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    use server::database::MEMDB;

    let data = MEMDB.lock().unwrap();
    data.get(&1)
        .map(|book| log::debug!("Got book {} from db.", book));
    Ok("Hello from the server!".to_string())
}

#[server(TestDbUsage)]
pub async fn test_db_usage() -> Result<String, ServerFnError> {
    let db_pool: &PgPool = DB.deref();
    let rs = sqlx::query("SELECT 1")
        .execute(db_pool)
        .await
        .map_err(|err| err.to_string());
    match rs {
        Ok(rs) => {
            let msg = format!("{} rows affected.", rs.rows_affected());
            log::debug!("{}", msg);
            Ok(msg)
        }
        Err(err) => {
            let msg = format!("Failed to use db due to '{}'.", err);
            log::debug!("{}", msg);
            Err(ServerFnError::Response(msg))
        }
    }
}
