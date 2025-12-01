use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        div { id: "dogview",
            img {
                src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg",
                max_height: "300px",
            }
        }
        div { id: "buttons", class: "flex flex-row gap-2",
            button { id: "skip", class: "cursor-pointer", "skip" }
            button { id: "save", class: "cursor-pointer", "save!" }
        }
    }
}
