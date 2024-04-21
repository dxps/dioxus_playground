use crate::server::functions::{get_permissions, get_user_name, login, logout};
use crate::ui::routes::Route;

use dioxus::prelude::*;

pub fn Home() -> Element {
    let mut user_name = use_signal(|| "?".to_string());
    let mut permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div {
                    table { class: "bg-white rounded-md",
                        tr {
                            td { class: "pl-4 py-2", colspan: 2,
                                button {
                                    class: "bg-green-50 drop-shadow-sm px-2 py-1 rounded-md",
                                    onclick: move |_| { async move { login().await.unwrap(); } },
                                    "Login Test User"
                                }
                            }
                        }
                        tr {
                            td { class: "pl-4 pl-2 py-2",
                                button {
                                    class: "bg-blue-50 px-2 py-1 rounded-md",
                                    onclick: move |_| async move {
                                        if let Ok(data) = get_user_name().await { user_name.set(data); }
                                    },
                                    "Get User Name"
                                }
                            }
                            td { class: "pl-2 pr-4", "User name: {user_name}" }
                        }
                        tr {
                            td { class: "pl-4 pr-2  py-2",
                                button {
                                    class: "bg-blue-50 px-2 py-1 rounded-md",
                                    onclick: move |_| async move {
                                        if let Ok(data) = get_permissions().await { permissions.set(data); }
                                    },
                                    "Get Permissions"
                                }
                            }
                            td { class: "pl-2 pr-4", "Permissions: {permissions}" }
                        }
                        tr {
                            td { class: "pl-4 py-2", colspan: "2",
                                button {
                                    class: "bg-gray-100 px-2 py-1 rounded-md",
                                    onclick: move |_| { async move { logout().await.unwrap(); } },
                                    "Logout Test User"
                                }
                            }
                        }
                    }
                }
                div { class: "pt-8",
                    div { class: "p-4",
                        Link { class: "pr-6", to: Route::Blog { id: 123 }, "Go to Blog" }
                        Link { to: Route::Sample {}, "Go to Sample" }
                    }
                }
            }
        }
    }
}
