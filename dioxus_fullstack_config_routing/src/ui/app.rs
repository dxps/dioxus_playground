use dioxus::prelude::*;

use crate::ui::routes::Route;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
