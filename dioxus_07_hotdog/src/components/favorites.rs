use crate::backend::{api_unfav_dog, list_dogs};
use dioxus::prelude::*;
use std::vec;

async fn fetch_favs(mut favs: Signal<Vec<(usize, String)>>) {
    match list_dogs().await {
        Result::Ok(entries) => {
            favs.set(entries);
        }
        Err(e) => {
            error!("Error fetching favorites: {e}");
        }
    }
}

#[component]
pub fn Favorites() -> Element {
    //
    let favs = use_signal::<Vec<(usize, String)>>(|| Vec::default());
    let mut initial = use_signal(|| true);

    // The initial fetch.
    use_resource(move || async move {
        if initial() {
            fetch_favs(favs).await;
            initial.set(false);
            debug!("Initially fetched {} favorites.", favs().len());
        }
    });

    let mut refetch_action = use_action(move || async move {
        fetch_favs(favs).await;
        debug!("Refetched {} favorites ...", favs().len());
        dioxus::Ok(())
    });

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favs() {
                    div { key: "{id}", class: "favorite-dog",
                        div { class: "relative",
                            img { class: "block w-full", src: "{url}" }
                            button {
                                class: "absolute m-auto rounded-full bg-red-700 text-white px-2 cursor-pointer",
                                onclick: move |_| {
                                    async move {
                                        match api_unfav_dog(id).await {
                                            Ok(()) => {
                                                refetch_action.call();
                                            }
                                            Err(e) => error!("Error unsaving dog {id}: {e}"),
                                        }
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
