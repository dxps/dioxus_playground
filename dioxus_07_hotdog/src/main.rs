mod backend;
mod components;

use components::*;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new axum router for our Dioxus app.
        let router = dioxus::server::router(App);

        // Customize it however you want.

        // And then return it.
        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
