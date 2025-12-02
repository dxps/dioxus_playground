use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new axum router for our Dioxus app
        let router = dioxus::server::router(App);

        // .. customize it however you want ..

        // And then return it
        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src, max_height: "300px" }
        }
        div { id: "buttons", class: "flex flex-row gap-2",
            button {
                id: "skip",
                onclick: move |_| img_src.restart(),
                class: "cursor-pointer",
                "skip"
            }
            button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = api_save_dog(current).await;
                },
                class: "cursor-pointer",
                "save!"
            }
        }
    }
}

#[post("/api/save_dog")]
async fn api_save_dog(image: String) -> Result<()> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist.
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url.
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}
