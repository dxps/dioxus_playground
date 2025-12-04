use dioxus::prelude::*;

use crate::backend::list_dogs;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // `use_server_future` is very similar to `use_resource`, but it waits for the future
    // to finish before continuing rendering and integrates with dioxus fullstack
    // to serialize that data from the server to the client.
    let favorites = use_server_future(list_dogs)?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorites().unwrap().unwrap() {
                    // Render a div for each photo using the dog's ID as the list key
                    div { key: "{id}", class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
