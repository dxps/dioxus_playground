use crate::server::functions::{get_permissions, get_user_name, login, logout};
use dioxus::prelude::*;

pub fn app() -> Element {
    let mut user_name = use_signal(|| "?".to_string());
    let mut permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            button { onclick: move |_| {
                    async move {
                        login().await.unwrap();
                    }
                },
                "Login Test User"
            }
        }
        div {
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_user_name().await {
                        user_name.set(data);
                    }
                },
                "Get User Name"
            }
            "User name: {user_name}"
        }
        div {
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_permissions().await {
                        permissions.set(data);
                    }
                },
                "Get Permissions"
            }
            "Permissions: {permissions}"
        }
        div {
            button { onclick: move |_| {
                    async move {
                        logout().await.unwrap();
                    }
                },
                "Logout Test User"
            }
        }
    }
}
