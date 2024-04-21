use crate::ui::routes::Route;

use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white px-12 py-6 rounded-md",
                    "Home"
                }
                div { class: "pt-8",
                    div { class: "p-4",
                        Link { class: "pr-6", to: Route::Blog { id: 123 }, "Go to Blog" }
                        Link { to: Route::Sample {}, "Go to Sample" }
                    }
                }
            }
        }
    }
}
