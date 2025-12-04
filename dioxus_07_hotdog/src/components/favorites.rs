use dioxus::prelude::*;

use crate::backend::{api_unsave_dog, list_dogs};

#[component]
pub fn Favorites() -> Element {
    //
    let mut favs = use_signal::<Vec<(usize, String)>>(|| Vec::default());
    let mut fetch = use_signal(|| false);

    // Initial fetch.
    // Create a pending resource that resolves to the list of dogs from the backend
    // `use_server_future` is very similar to `use_resource`, but it waits
    // for the future to finish before continuing rendering and integrates
    // with dioxus fullstack to serialize that data from the server to the client.
    let favorites = use_server_future(list_dogs)?;
    favs.set(favorites.unwrap().unwrap());

    // let mut refetch_action = use_action(move || async move {
    // info!("Refetching favorites ...");
    // match list_dogs().await {
    // Ok(entries) => {
    // info!("Refetched {} favorites.", entries.len());
    // favs.set(entries);
    // Ok(())
    // }
    // Err(e) => {
    // error!("Error refetching favorites: {e}");
    // Err(e)
    // }
    // }
    // });

    use_effect(move || {
        if fetch() {
            spawn(async move {
                info!("Refetching favorites ...");
                match list_dogs().await {
                    Ok(entries) => {
                        info!("Refetched {} favorites.", entries.len());
                        favs.set(entries);
                    }
                    Err(e) => {
                        error!("Error refetching favorites: {e}");
                    }
                }
            });
            fetch.set(false);
        }
    });

    info!("Rendering {} favorites ...", favs().len());

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favs() {
                    // Render a div for each photo using the dog's ID as the list key
                    div { key: "{id}", class: "favorite-dog",
                        div { class: "relative",
                            img { class: "block w-full", src: "{url}" }
                            button {
                                class: "absolute m-auto rounded-full bg-red-700 text-white px-2 cursor-pointer",
                                onclick: move |_| {
                                    async move {
                                        let _ = api_unsave_dog(id).await;
                                        fetch.set(true);
                                    }
                                },
                                "x"
                            }
                        }
                    }
                }
            }
        }
    }
}
