#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use tracing::{debug, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("Failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="/styles.css">"#.to_string());

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    //
    let onsubmit = move |data: HashMap<String, FormValue>| {
        debug!("[Home] [onsubmit] data: {:?}", data);
    };
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6 mt-8 mb-8 min-w-[600px]",
                    AddPublisherForm { onsubmit }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct AddPublisherFormProps {
    onsubmit: EventHandler<HashMap<String, FormValue>>,
}

#[component]
fn AddPublisherForm(props: AddPublisherFormProps) -> Element {
    let onsubmit = move |evt: Event<FormData>| {
        let vals = evt.data.values();
        let name = vals.get("name");

        let name = name.unwrap().as_value();
        debug!("Creating user with name '{name}' ...");

        spawn(async move {
            debug!("Inside the spawn, creating user with name '{name}' ...");
            // ...
        });

        let data = evt.data().values();
        props.onsubmit.call(data);
    };

    rsx! {
        div {
            form { class: "flex flex-row", onsubmit: onsubmit,

                fieldset { class: "flex flex-row",
                    legend { "Add publisher" }

                    input {
                        class: "border-2 border-gray-200 rounded px-2 py-1 w-full",
                        name: "name",
                        placeholder: "Enter name"
                    }

                    "male"

                    input {
                        r#type: "radio",
                        name: "gender",
                        value: "m",
                        checked: true
                    }

                    "female"

                    input {
                        r#type: "radio",
                        name: "gender",
                        value: "f",
                        checked: false
                    }

                    "suspended"
                    input {
                        r#type: "checkbox",
                        name: "suspended",
                        checked: false
                    }

                    button { class: "text-medium bg-slate-100 hover:bg-slate-300 py-1 px-2 rounded cursor-pointer",
                        "Create"
                    }
                }
            }
        }
    }
}
