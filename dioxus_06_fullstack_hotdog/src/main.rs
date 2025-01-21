mod app_error;

use app_error::AppError;

#[cfg(feature = "server")]
// See https://dioxuslabs.com/learn/0.6/guides/use axum::routing::get;
use axum;
use dioxus::prelude::*;

#[cfg(feature = "server")]
// See https://dioxuslabs.com/learn/0.6/guides/
use dioxus_cli_config;

#[cfg(feature = "server")]
use sqlx::Row;

#[cfg(feature = "server")]
// See https://dioxuslabs.com/learn/0.6/guides/fullstack/managing_dependencies/
use tokio;

#[cfg(feature = "server")]
use sqlx::{postgres::PgPoolOptions, PgPool};

const CSS: Asset = asset!("/assets/main.css");

// Create a new wrapper type
#[derive(Clone)]
struct TitleState(String);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "server")]
    dotenvy::dotenv()?;

    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    Ok(())
}

/////////////////////////////////////////
// Server elements (functions & logic) //
////////////////////////////////////////

#[cfg(feature = "server")]
pub static DB_POOL: std::sync::LazyLock<PgPool> =
    std::sync::LazyLock::new(|| init_dbpool().unwrap());

// #[cfg(feature = "server")]
// thread_local! {
// #[allow(non_upper_case_globals)]
// pub static dbp: PgPool;
// }

#[cfg(feature = "server")]
async fn launch_server() {
    // Connect to dioxus' logging infrastructure.
    dioxus::logger::initialize_default();

    init_logging();
    log::info!("Starting up the server ...");

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile).
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    // Build a custom Axum router.
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "server")]
fn init_logging() {
    use log::LevelFilter::{Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", Info)
        .with_module_level("tungstenite", Info)
        .with_module_level("tokio_tungstenite", Info)
        .with_module_level("axum_session", Info)
        .with_module_level("axum_session_auth", Warn)
        .with_module_level("dioxus_core", Warn)
        .with_module_level("dioxus_signals", Info)
        .with_module_level("tracing", Warn)
        .init()
        .unwrap();
}

#[cfg(feature = "server")]
pub fn init_dbpool() -> Result<PgPool, AppError> {
    //
    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!(
            "Unknown DATABASE_URL environment variable. Reason: '{}'.",
            err
        );
        AppError::Err("Unknown DATABASE_URL environment variable".into())
    })?;

    let db_conn_options: sqlx::postgres::PgConnectOptions = db_url
        .parse()
        .expect("Failed to parse database URL '{db_url}'.");

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect_lazy_with(db_conn_options);
    log::info!("Database connection pool created.");

    Ok(pool)
}

// Expose a `save_dog` endpoint on our server that takes an "image" parameter.
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    _ = file.write_fmt(format_args!("{image}\n"));

    log::info!("URL '{image}' was saved to 'dogs.txt' file.");

    Ok(())
}

#[server]
async fn save_dog_to_db(_image_url: String) -> Result<(), ServerFnError> {
    // For now, just testing the db interaction.
    let mut conn = DB_POOL.acquire().await.unwrap();
    log::info!("Number of active db connections: {}.", DB_POOL.size());
    let row = sqlx::query("SELECT to_char(current_timestamp, 'YYYY-MM-DD HH24:MI:SS') now")
        .fetch_one(conn.as_mut())
        .await
        .unwrap();
    log::info!("Database time: '{}'.", row.get::<&str, &str>("now"));

    Ok(())
}

////////////////
// Components //
////////////////

#[component]
fn App() -> Element {
    // Provide that type as a Context
    use_context_provider(|| TitleState("HotDog! 🌭".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "flex flex-col min-h-screen bg-gray-300 justify-center items-center",
            Title {}
            DogView {}
        }
    }
}

#[component]
fn Title() -> Element {
    // Consume that type as a Context.
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title", class: "pb-12",
            p { class: "text-3xl text-gray-500 font-bold", "{title.0}" }
        }
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview", class: "flex justify-center mb-4 space-x-4",
            button {
                class: "bg-gray-300 hover:bg-gray-600 hover:text-white rounded-md px-2",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current.clone()).await;
                    _ = save_dog_to_db(current).await;
                },
                id: "save",
                "Save"
            }
            button {
                class: "bg-gray-300 hover:bg-gray-600 hover:text-white rounded-md px-3",
                onclick: move |_| img_src.restart(),
                "id": "refresh",
                "Refresh"
            }
        }
        div { id: "dogimage",
            img {
                class: "rounded-lg h-96",
                src: img_src.cloned().unwrap_or_default(),
            }
        }
    }
}
