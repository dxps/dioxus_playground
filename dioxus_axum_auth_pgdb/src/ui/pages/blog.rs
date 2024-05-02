use crate::ui::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-row min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md m-2 p-2 space-y-3",
                    div {
                        "Blog post {id}"
                    }
                    div {
                        Link { to: Route::Home {}, "Back to home" }
                    }
                }
            }
        }
    }
}
