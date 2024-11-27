#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

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

#[component]
fn SortableList(items: Signal<Vec<(i32, String)>>) -> Element {
    //
    let mut drag_source_item = use_signal(|| 0);
    let mut drag_target_item = use_signal(|| 0);

    // React on changes.
    use_memo(move || {
        info!(
            ">>> React on changes: drag_source_item: {} drag_target_item: {}",
            drag_source_item(),
            drag_target_item()
        );
    });

    rsx! {
        div { class: "mb-4 flex flex-col items-center",
            div { class: "text-gray-500", "Drag and drop items to reorder: " }
            div { class: "text-gray-700", " {drag_source_item} -> {drag_target_item}" }
        }
        ul { class: "list-disc ml-4",
            li {
                draggable: true,
                ondragstart: move |e| {
                    e.stop_propagation();
                    drag_source_item.set(1);
                },
                ondragover: move |e| {
                    e.stop_propagation();
                    drag_target_item.set(1);
                },
                "Item 1"
            }
            li {
                draggable: true,
                ondragstart: move |e| {
                    e.stop_propagation();
                    drag_source_item.set(2);
                },
                ondragover: move |e| {
                    e.stop_propagation();
                    drag_target_item.set(2);
                },
                "Item 2"
            }
            li {
                draggable: true,
                ondragstart: move |e| {
                    e.stop_propagation();
                    drag_source_item.set(3);
                },
                ondragover: move |e| {
                    e.stop_propagation();
                    drag_target_item.set(3);
                },
                "Item 3"
            }
            li {
                draggable: true,
                ondragstart: move |e| {
                    e.stop_propagation();
                    drag_source_item.set(4);
                },
                ondragover: move |e| {
                    e.stop_propagation();
                    drag_target_item.set(4);
                },
                "Item 4"
            }
        }
    }
}
