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
    // `IndexMap` is a map that keeps the insertion order.
    // Those items ids are just for reflecting a real world case where
    // the items have their own id, these having nothing to do with the order.
    //
    let mut items = IndexMap::from([
        ("id-1".to_string(), "First Item".to_string()),
        ("id-2".to_string(), "Second Item".to_string()),
        ("id-3".to_string(), "Third Item".to_string()),
        ("id-4".to_string(), "Fourth Item".to_string()),
    ]);

    let mut reordered_items = use_signal(|| IndexMap::<String, String>::from(items.clone()));
    let order_change = use_signal(|| (0, 0));
    let dragging_in_progress = use_signal(|| false);

    use_effect(move || {
        let (source_index, target_index) = order_change();
        if !dragging_in_progress() && source_index != target_index {
            let mut changed_items = items.clone();
            let range: &mut dyn Iterator<Item = usize> = if source_index < target_index {
                &mut (source_index..target_index)
            } else {
                &mut (target_index..source_index).rev().into_iter()
            };
            for index in range {
                info!(">>> [Home] Swapping {} <-> {}", index, index + 1);
                changed_items.swap_indices(index, index + 1);
            }
            items = changed_items.clone();
            reordered_items.set(changed_items);
        }
    });

    rsx! {
        div {
            div { class: "flex flex-col min-h-screen bg-gray-100",
                div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                    div { class: "bg-white rounded-md p-6 mt-8 mb-8 w-[600px] min-h-[300px]",
                        p { class: "text-2xl text-gray-500 font-medium text-center",
                            "Drag and Drop List"
                        }
                        hr { class: "mb-4" }
                        DnDList { items: reordered_items, order_change, dragging_in_progress }
                    }
                }
            }
        }
    }
}
