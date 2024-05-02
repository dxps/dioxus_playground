#![allow(non_snake_case, unused)]

#[cfg(feature = "server")]
mod auth;
mod server;

mod ui;

fn main() {
    //
    #[cfg(feature = "web")]
    // Hydrate the application on the client.
    dioxus_web::launch::launch_cfg(ui::app, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    server::server_start(ui::app)
}
