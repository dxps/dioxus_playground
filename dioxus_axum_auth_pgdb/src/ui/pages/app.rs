use crate::server::functions::{get_permissions, get_user_name, login, logout};
use dioxus::prelude::*;

pub fn app() -> Element {
    let mut user_name = use_signal(|| "?".to_string());
    let mut permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-row min-h-screen justify-center items-center drop-shadow-2xl",
                table { class: "bg-white rounded-md m-2",
                    tr {
                        td { class: "m-2 p-2",
                            button {
                                class: "bg-green-50 drop-shadow-sm px-2 py-1 rounded-md",
                                onclick: move |_| { async move { login().await.unwrap(); } },
                                "Login Test User"
                            }
                        }
                        td {}
                    }
                    tr {
                        td { class: "m-2 p-2",
                            button {
                                class: "bg-blue-50 px-2 py-1 rounded-md",
                                onclick: move |_| async move {
                                    if let Ok(data) = get_user_name().await { user_name.set(data); }
                                },
                                "Get User Name"
                            }
                        }
                        td { "User name: {user_name}" }
                    }
                    tr {
                        td { class: "m-2 p-2",
                            button {
                                class: "bg-blue-50 px-2 py-1 rounded-md",
                                onclick: move |_| async move {
                                    if let Ok(data) = get_permissions().await {
                                        permissions.set(data);
                                    }
                                },
                                "Get Permissions"
                            }
                        }
                        td { "Permissions: {permissions}" }
                    }
                    tr {
                        td { class: "m-2 p-2", colspan: "2",
                            button {
                                class: "bg-gray-100 px-2 py-1 rounded-md",
                                onclick: move |_| { async move { logout().await.unwrap(); } },
                                "Logout Test User"
                            }
                        }
                    }
                }
            }
        }
    }
}
