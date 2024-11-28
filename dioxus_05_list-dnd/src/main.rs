#![allow(non_snake_case)]

mod sortable_list;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use sortable_list::SortableList;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

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
    let items = use_signal(|| {
        vec![
            (1, "Item 1".to_string()),
            (2, "Item 2".to_string()),
            (3, "Item 3".to_string()),
            (4, "Item 4".to_string()),
        ]
    });

    rsx! {
        div {
            div { class: "flex flex-col min-h-screen bg-gray-100",
                div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                    div { class: "bg-white rounded-md p-6 mt-8 mb-8 w-[600px]",
                        h2 { class: "text-3xl text-gray-500 font-medium text-center",
                            "HTML List Drag-n-Drop"
                        }
                        hr { class: "mb-8" }
                        SortableList { items }
                    }
                }
            }
        }
    }
}
