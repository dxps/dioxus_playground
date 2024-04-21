use dioxus::prelude::*;

use crate::ui::routes::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div {
                    class: "bg-white p-4 rounded-md",
                    "Blog post id {id}"
                }
                div { class: "pt-12",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
