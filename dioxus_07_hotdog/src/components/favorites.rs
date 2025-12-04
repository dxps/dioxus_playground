use dioxus::prelude::*;

use crate::backend::list_dogs;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let favorites = use_resource(list_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorites().unwrap() {
                    // Render a div for each photo using the dog's ID as the list key
                    div { key: "{id}", class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
