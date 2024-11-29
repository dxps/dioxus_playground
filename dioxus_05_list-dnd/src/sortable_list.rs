use dioxus::prelude::*;
use dioxus_elements::source;
use dioxus_logger::tracing::info;

#[component]
pub fn SortableList(items: Signal<Vec<(u32, String)>>) -> Element {
    //
    let mut drag_source_index = use_signal(|| 0u32);
    let mut drag_target_index = use_signal(|| 0u32);

    // Have its own copy to prevent an infinite loop (that would happen
    // when reading and writing to the same signal in the same scope)
    // and two scopes (use_effect hooks) are used.
    let mut my_items = use_signal(|| Vec::from(items()));

    use_effect(move || {
        let source_index = drag_source_index();
        let target_index = drag_target_index();
        if source_index != target_index && source_index > 0 && target_index > 0 {
            info!(
                ">>> React on changes: source_index: {} target_index: {}",
                source_index, target_index
            );
            my_items.set(items());
        }
    });

    use_effect(move || {
        let source_index = usize::try_from(drag_source_index()).unwrap();
        let target_index = usize::try_from(drag_target_index()).unwrap();
        if source_index != target_index {
            let mut new_items = items();

            if source_index < target_index {
                // Dragging down.
                let source_item = new_items.get(source_index).unwrap().clone();
                new_items.remove(source_index);
                if target_index < new_items.len() {
                    new_items.insert(target_index, source_item);
                } else {
                    new_items.push(source_item);
                }
            } else {
                // Dragging up.
            }
            // Swap items.
            // reordered_items.swap(
            //     usize::try_from(drag_source_item() - 1).unwrap(),
            //     usize::try_from(drag_target_item() - 1).unwrap(),
            // );
            info!(">>> React on changes: newitems: {:?} ", new_items);
            my_items.set(new_items);
        }
    });

    rsx! {
        div { class: "mb-4 flex flex-col items-center",
            div { class: "text-gray-500", "Drag and drop items to reorder: " }
            div { class: "text-gray-700", " {drag_source_index} -> {drag_target_index}" }
        }
        ul { class: "list-disc ml-4",
            for item in my_items() {
                li {
                    draggable: true,
                    ondragstart: move |e| {
                        e.stop_propagation();
                        drag_source_index.set(item.0 - 1);
                    },
                    ondragover: move |e| {
                        e.stop_propagation();
                        drag_target_index.set(item.0 - 1);
                    },
                    "{item.1}"
                }
            }
        }
    }
}
