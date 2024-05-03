pub mod pages;

pub mod routes;

use crate::ui::routes::Route;
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
