use crate::backend::{api_unfav_dog, list_dogs};
use dioxus::prelude::*;
use std::vec;

#[component]
pub fn Favorites() -> Element {
    //
    let mut favs = use_signal::<Vec<(usize, String)>>(|| Vec::default());
    let mut initial = use_signal(|| true);

    // The initial fetch.
    use_resource(move || async move {
        if initial() {
            let entries = list_dogs().await;
            favs.set(entries.unwrap());
            initial.set(false);
            info!("Initial fetch of {} favorites.", favs().len());
        }
    });

    let mut refetch_action = use_action(move || async move {
        info!("Refetching favorites ...");
        match list_dogs().await {
            Result::Ok(entries) => {
                info!("Refetched {} favorites.", entries.len());
                favs.set(entries);
            }
            Err(e) => {
                error!("Error refetching favorites: {e}");
            }
        }
        dioxus::Ok(())
    });

    rsx! {
        p { "{favs().len()} Favorites" }
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favs() {
                    // Render a div for each photo using the dog's ID as the list key.
                    div { key: "{id}", class: "favorite-dog",
                        div { class: "relative",
                            img { class: "block w-full", src: "{url}" }
                            button {
                                class: "absolute m-auto rounded-full bg-red-700 text-white px-2 cursor-pointer",
                                onclick: move |_| {
                                    async move {
                                        match api_unfav_dog(id).await {
                                            Ok(()) => {
                                                info!("Unfaved dog {id}");
                                                refetch_action.call();
                                            }
                                            Err(e) => error!("Error unsaving dog {id}: {e}"),
                                        }
                                        refetch_action.call();
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
