pub mod pages;

pub mod routes;

pub mod ui_global_state;

use crate::ui::routes::Route;
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
