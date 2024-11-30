use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use indexmap::IndexMap;

#[component]
pub fn DnDList(
    items: Signal<IndexMap<String, String>>,
    order_change: Signal<(usize, usize)>,
    dragging_in_progress: Signal<bool>,
) -> Element {
    //
    let mut drag_source_index = use_signal(|| 0usize);
    let mut drag_target_index = use_signal(|| 0usize);

    use_effect(move || {
        let source_index = drag_source_index();
        let target_index = drag_target_index();
        order_change.set((source_index, target_index));
    });

    rsx! {
        div { class: "mb-4 flex flex-col items-center",
            div { class: "text-gray-500", "Drag and drop items to reorder." }
            div { class: "text-gray-400 text-xs m-h-4",
                if dragging_in_progress() {
                    p { "Dragging indices: {drag_source_index} -> {drag_target_index}" }
                } else {
                    p { dangerous_inner_html: "&nbsp;" }
                }
            }
        }
        ul { class: "list-disc ml-4",
            for (index , item) in items().iter().enumerate() {
                li {
                    draggable: true,
                    ondragstart: move |_| {
                        drag_source_index.set(index);
                        dragging_in_progress.set(true);
                    },
                    ondragover: move |_| {
                        if index != drag_target_index() {
                            drag_target_index.set(index);
                        }
                    },
                    ondragend: move |_| {
                        dragging_in_progress.set(false);
                        info!(">>> [DnDList] ondragend: {drag_source_index} -> {drag_target_index}");
                    },
                    "{item.1}"
                }
            }
        }
    }
}
