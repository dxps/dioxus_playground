#![allow(non_snake_case)]

mod dnd_list;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
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
                changed_items.swap_indices(index, index + 1);
            }
            items = changed_items.clone();
            new_items.set(changed_items);
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
                        DnDList { items: new_items, order_change, dragging_in_progress }
                    }
                }
            }
        }
    }
}
