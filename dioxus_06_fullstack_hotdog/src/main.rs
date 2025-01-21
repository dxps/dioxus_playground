#[cfg(feature = "server")]
// See https://dioxuslabs.com/learn/0.6/guides/use axum::routing::get;
use axum;
use dioxus::prelude::*;

#[cfg(feature = "server")]
// See https://dioxuslabs.com/learn/0.6/guides/
use dioxus_cli_config;

#[cfg(feature = "server")]
// See https://dioxuslabs.com/learn/0.6/guides/fullstack/managing_dependencies/
use tokio;

const CSS: Asset = asset!("/assets/main.css");

// Create a new wrapper type
#[derive(Clone)]
struct TitleState(String);

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    // Connect to dioxus' logging infrastructure.
    dioxus::logger::initialize_default();

    init_logging();
    log::info!("Starting up the server ...");

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile).
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    // Build a custom axum router.
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
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
                    _ = save_dog(current).await;
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
        div { id: "dogview",
            img {
                class: "rounded-lg h-96",
                src: img_src.cloned().unwrap_or_default(),
            }
        }
    }
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
