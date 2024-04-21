use dioxus::prelude::*;

use crate::server::fns::sample::{get_server_data, post_server_data};
use crate::ui::routes::Route;

#[component]
pub fn Sample() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white p-4 rounded-md",
                    h1 { "High-Five counter: {count}" }
                    button { class: "bg-slate-300 rounded-lg m-2 px-2 py-1", onclick: move |_| count += 1, " + " }
                    button { class: "bg-slate-400 rounded-lg m-2 px-2 py-1", onclick: move |_| count -= 1, " - " }
                }
                div { class: "bg-white mt-6 p-4 rounded-md",
                    button { class: "bg-slate-100 rounded-lg m-2 px-2 py-1",
                        onclick: move |_| async move {
                            if let Ok(data) = get_server_data().await {
                                println!("Client received: {}", data);
                                text.set(data.clone());
                                post_server_data(data).await.unwrap();
                            }
                        },
                        "Get Server Data"
                    }
                    p { "Server data: {text}"}
                }
                div { class: "pt-12",
                    Link { class: "pr-6", to: Route::Blog { id: count() }, "Go to Blog id {count}"}
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
