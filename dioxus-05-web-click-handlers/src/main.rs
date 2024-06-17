#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::debug;

fn main() {
    // Init logger.
    _ = console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

    launch(App);
}

#[component]
fn App() -> Element {
    //
    debug!(">>> App rendered.");

    rsx! {
        div {
            "style": "width: 100%; height: 1000%; background-color: lightgray; padding: 100px; position: absolute; top: 0; left: 0; z-index: 40",
            onclick: move |_| {
                debug!(">>> Clicked in the outer.");
            },
            { "Outer div" },
            div { "style": "float: right; width: 24rem; margin-right: 0.5rem; background-color: white;
                            border-radius: 0.5rem",
                div {
                    ul { "style": "list-style-type: none",
                        li { "style": "color: #aaa; padding-bottom: 12px", "user menu" }
                        li { "style": "color: #333; cursor: pointer; background-color: #eee;
                                       padding: 4px 8px 4px 8px; margin-bottom:12px; border-radius: 0.5rem",
                            "My Profile"
                        }
                        li {
                            "style": "color: #333; cursor: pointer; background-color: #eee;
                                       padding: 4px 8px 4px 8px; border-radius: 0.5rem",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                debug!(">>> Clicked on Logout.");
                            },
                            "Logout"
                        }
                    }
                }
            }
        }
    }
}
