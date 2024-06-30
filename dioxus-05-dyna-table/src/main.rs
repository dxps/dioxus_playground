#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    //

    let mut entries = use_signal(|| vec!["Entry 1".to_string()]);

    rsx! {
        div {
            div { class: "flex flex-col min-h-screen bg-gray-100",
                div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                    div { class: "bg-white rounded-md p-6 mt-8 mb-8 w-[600px]",
                        h2 { class: "text-3xl text-gray-500 font-medium text-center",
                            "Dynamic Table"
                        }
                        div { class: "flex flex-row justify-between mt-6",
                            p { class: "text-gray-500",
                                "The case when one or more entries needs to be managed."
                            }
                            p {
                                class: "text-medium bg-slate-50 hover:bg-green-200 py-1 px-2 rounded-lg cursor-pointer",
                                onclick: move |_| {
                                    let id = entries.read().len();
                                    entries.write().push(format!("Entry {}", id + 1));
                                },
                                "+"
                            }
                        }
                        hr { class: "mt-4 mb-3 h-2" }

                        for (idx , entry) in entries.read().iter().enumerate() {
                            div { class: "flex flex-row justify-between hover:bg-slate-100 rounded",
                                p { class: "px-2 py-1", "{entry}" }
                                p {
                                    class: "text-slate-300 hover:text-red-600 hover:bg-red-100 px-2 py-1 rounded-lg cursor-pointer",
                                    onclick: move |_| {
                                        entries.write().remove(idx);
                                    },
                                    "x"
                                }
                            }
                        }

                        hr { class: "my-4 h-2" }
                    }
                }
            }
        }
    }
}
