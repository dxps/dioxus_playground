use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn SortableList(items: Signal<Vec<(u32, String)>>) -> Element {
    //
    let mut drag_source_item = use_signal(|| 0u32);
    let mut drag_target_item = use_signal(|| 0u32);

    // Have its own copy to prevent an infinite loop (that would happen
    // when reading and writing to the same signal in the same scope)
    // and two scopes (use_effect hooks) are used.
    let mut my_items = use_signal(|| Vec::from(items()));

    use_effect(move || {
        let source_item = drag_source_item();
        let target_item = drag_target_item();
        if source_item != target_item && source_item > 0 && target_item > 0 {
            info!(
                ">>> React on changes: drag_source_item: {} drag_target_item: {}",
                drag_source_item(),
                drag_target_item()
            );
            my_items.set(items());
        }
    });

    use_effect(move || {
        let source_item = drag_source_item();
        let target_item = drag_target_item();
        if source_item != target_item && source_item > 0 && target_item > 0 {
            // Swap items.
            let mut reordered_items = items();
            reordered_items.swap(
                usize::try_from(drag_source_item() - 1).unwrap(),
                usize::try_from(drag_target_item() - 1).unwrap(),
            );
            my_items.set(reordered_items);
        }
    });

    rsx! {
        div { class: "mb-4 flex flex-col items-center",
            div { class: "text-gray-500", "Drag and drop items to reorder: " }
            div { class: "text-gray-700", " {drag_source_item} -> {drag_target_item}" }
        }
        ul { class: "list-disc ml-4",
            for item in my_items() {
                li {
                    draggable: true,
                    ondragstart: move |e| {
                        e.stop_propagation();
                        drag_source_item.set(item.0);
                    },
                    ondragover: move |e| {
                        e.stop_propagation();
                        drag_target_item.set(item.0);
                    },
                    "{item.1}"
                }
            }
        }
    }
}
