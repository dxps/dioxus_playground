use dioxus::prelude::*;

use crate::{DogApi, backend::sf_fav_dog};

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let skip_dog = move |_| img_src.restart();

    let fav_dog = move |_| async move {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        _ = sf_fav_dog(current).await;
    };

    rsx! {
        div { id: "dogview",
            img { src: img_src }
        }
        div { id: "buttons", class: "flex flex-row gap-2",
            button { id: "skip", onclick: skip_dog, class: "cursor-pointer", "skip" }
            button { id: "save", onclick: fav_dog, class: "cursor-pointer", "save!" }
        }
    }
}

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");

    rsx! {
        div { "Page not found. Path: {path}" }
    }
}
