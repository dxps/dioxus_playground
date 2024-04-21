use dioxus::prelude::*;

use crate::ui::pages::{Blog, Home, Sample};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/sample")]
    Sample {},

    // The catch all routes that will match any route
    // and placing the matched segments in the route field.
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
