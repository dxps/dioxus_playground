#![allow(non_snake_case)]

mod state;

use dioxus::prelude::*;
use state::State;
use tracing::debug;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(tracing::Level::DEBUG).expect("Failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    //
    let mut state = use_context_provider(|| Signal::new(None::<State>));

    // Asynchronously loading the state from localstorage and notify its value through the signal.
    use_future(move || async move {
        let mut state_signal = use_context::<Signal<Option<State>>>();

        // Using this "standard" usage (with `dioxus_sdk`), it crashes with:
        // ```
        // panicked at /home/dxps/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dioxus-core-0.5.1/src/global_context.rs:126:64:
        // to be in a dioxus runtime
        // ```

        match State::load_from_localstorage() {
            Ok(state) => *state_signal.write() = Some(state),
            Err(err) => {
                debug!(">>> [Home] Error: {err}!");
            }
        }
    });

    rsx! {
        div {
            { "State: " },
            if let Some(state) = state.read().as_ref() {
                if let Some(something) = state.something.as_ref() {
                    "{something}"
                } else {
                    "None"
                }
            } else {
                p { "Loading..." }
            }
        }
        hr {}
        div {
            button {
                class: "bg-green-100 hover:bg-green-200 rounded px-2 py-1",
                onclick: move |_| {
                    state.write().as_mut().unwrap().something = Some("something".to_string());
                    state.write().as_mut().unwrap().save_to_localstorage();
                },
                "Set state"
            }
            button {
                class: "bg-red-100 hover:bg-red-200 rounded px-2 py-1",
                onclick: move |_| {
                    state.write().as_mut().unwrap().something = None;
                    state.write().as_mut().unwrap().save_to_localstorage();
                },
                "Clear state"
            }
        }
    }
}
