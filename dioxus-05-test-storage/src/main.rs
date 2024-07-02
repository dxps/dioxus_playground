#![allow(non_snake_case)]

mod state;

use dioxus::prelude::*;
use state::State;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("Failed to init logger");
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
    let state = use_context_provider(|| Signal::new(State::default()));

    // Asynchronously loading the state from localstorage and notify its value through the signal.
    use_future(move || async move {
        let mut state = use_context::<Signal<State>>();
        let local_state = State::load_from_localstorage();
        *state.write() = local_state();
    });

    rsx! {
        div {
            h3 { "State: " }
            if let Some(something) = state.read().something.as_ref() {
                p { "{something}" }
            } else {
                p { "Loading..." }
            }
        }
    }
}
