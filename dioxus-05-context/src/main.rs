#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    //
    let count = use_context_provider(|| Signal::new(1));

    rsx! {
        small {
            "The counter is {count}."
        }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    //
    let mut count = use_context::<Signal<i32>>();
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| *count.write() += 1, "Up high!" }
            button { onclick: move |_| *count.write() -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
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

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
