use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    use crate::ui::routes::Route;

    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-4",
                    div { class: "text-xl px-2",
                        "Blog post {id}"
                    }
                }
                div { class: "pt-8",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}
