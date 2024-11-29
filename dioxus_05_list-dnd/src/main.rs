#![allow(non_snake_case)]

mod dnd_list;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dnd_list::DnDList;
use indexmap::IndexMap;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("Failed to init logger!");
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
    let mut items = IndexMap::from([
        ("0".to_string(), "First Item".to_string()),
        ("1".to_string(), "Second Item".to_string()),
        ("2".to_string(), "Third Item".to_string()),
        ("3".to_string(), "Fourth Item".to_string()),
    ]);

    let mut new_items = use_signal(|| IndexMap::<String, String>::from(items.clone()));
    let order_change = use_signal(|| (0, 0));

    use_effect(move || {
        let (source_index, target_index) = order_change();
        if source_index != target_index {
            let mut changed_items = items.clone();
            if source_index < target_index {
                for index in source_index..target_index {
                    info!(">>> [Home] Swapping {} and {}", index, index + 1);
                    changed_items.swap_indices(index, index + 1);
                }
                info!(">>> [Home] After swap, changed_items: {:?}", changed_items);
                items = changed_items.clone();
                new_items.set(changed_items);
            } else {
                // TODO
            }
        }
    });

    rsx! {
        div {
            div { class: "flex flex-col min-h-screen bg-gray-100",
                div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                    div { class: "bg-white rounded-md p-6 mt-8 mb-8 w-[600px]",
                        h2 { class: "text-3xl text-gray-500 font-medium text-center",
                            "Drag-n-Drop List"
                        }
                        hr { class: "mb-8" }
                        DnDList { items: new_items, order_change }
                    }
                }
            }
        }
    }
}
