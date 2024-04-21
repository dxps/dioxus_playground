#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/sample")]
    Sample {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "bg-gray-100",
            div {
                class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div {
                    "Blog post {id}"
                }
                div {
                    class: "pt-6",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}

fn Sample() -> Element {
    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div {
                    "Sample Page"
                }
                div {
                    Link { to: Route::Home {}, "Back to Home"}
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            class: "bg-gray-100",
            div {
                class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div {
                    class: "flex flex-row p-3 h-full",
                    Link {
                        class: "pr-4",
                        to: Route::Blog { id: count() },
                        "Go to Blog"
                    }
                    Link {
                        to: Route::Sample {},
                        "Go to Sample"
                    }
                }

                div {
                    class: "pt-6",
                    h1 { "High-Five counter: {count}" }
                    button {
                        class: "rounded-md px-2 py-1 bg-white rop-shadow-sm",
                        onclick: move |_| count += 1, "Up high!"
                    }
                    button {
                        class: "rounded-md ml-3 px-2 py-1 bg-white rop-shadow-sm",
                        onclick: move |_| count -= 1, "Down low!"
                    }
                }
                div {
                    class: "mt-6",
                    button {
                        class: "rounded-md ml-3 px-2 py-1 bg-white rop-shadow-sm",
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
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
