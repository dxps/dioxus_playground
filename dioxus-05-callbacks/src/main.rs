#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    init_logging();

    log::info!("starting app");
    launch(App);
}

fn App() -> Element {
    // _ = console_log::init_with_level(log::Level::Debug);
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

// --------------------------------------------------

#[derive(Props, PartialEq, Clone)]
struct IncreaseButtonProps {
    pub label: String,
    pub click_handler: EventHandler<MouseEvent>,
}

#[component]
fn IncreaseButton(props: IncreaseButtonProps) -> Element {
    let IncreaseButtonProps {
        label,
        click_handler,
    } = props;
    rsx! {
        button { onclick: move |evt| click_handler(evt), "IncreaseButton ({label}" }
    }
}

// --------------------------------------------------

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }

            IncreaseButton { label: "Up high!", click_handler: move |_| { count += 1 } }

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
            p { "Server data: {text}" }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    log::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

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
