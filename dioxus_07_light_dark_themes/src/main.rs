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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        ToggleLightDark {}
        Hero {}
        Echo {}
    }
}

#[component]
pub fn ToggleLightDark() -> Element {
    let mut dark = use_signal(|| true);

    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document.document_element().unwrap();

        if *dark.read() {
            root.class_list().add_1("dark").unwrap();
        } else {
            root.class_list().remove_1("dark").unwrap();
        }
    });

    rsx! {
        div { class: "p-6 bg-neutral-300 dark:bg-neutral-800 rounded-lg mb-4",
            button {
                class: "px-3 py-2 text-orange-700 dark:text-orange-400 bg-neutral-200 dark:bg-neutral-700 rounded-lg",
                onclick: move |_| dark.set(!dark()),
                if dark() {
                    "Switch to Light"
                } else {
                    "Switch to Dark"
                }
            }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
