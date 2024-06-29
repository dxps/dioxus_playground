#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to home" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    //
    let mut tab_to_show = use_signal(|| "primary_info".to_string());

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6 mt-24 mb-8",
                    h1 { class: "text-3xl text-[#333] font-bold text-center", "My Profile" }
                    // The tabs.
                    ul { class: "flex gap-4 bg-gray-100 rounded-lg my-4 p-[3.4px] w-max overflow-hidden font-sans mx-auto",
                        li {
                            class: if tab_to_show() == "primary_info".to_string() {
                                "text-green-600 font-medium rounded-lg text-center text-sm bg-white py-2 px-4 tracking-wide cursor-pointer"
                            } else {
                                "text-gray-600 rounded-lg text-center text-sm hover:bg-white hover:text-lilac py-2 px-4 tracking-wide cursor-pointer"
                            },
                            onclick: move |_| tab_to_show.set("primary_info".to_string()),
                            "Primary Info"
                        }
                        li {
                            class: if tab_to_show() == "security".to_string() {
                                "text-green-600 font-medium rounded-lg text-center text-sm bg-white py-2 px-4 tracking-wide cursor-pointer"
                            } else {
                                "text-gray-600 rounded-lg text-center text-sm hover:bg-white hover:text-lilac py-2 px-4 tracking-wide cursor-pointer"
                            },
                            onclick: move |_| tab_to_show.set("security".to_string()),
                            "Security"
                        }
                    }
                    if tab_to_show() == "primary_info".to_string() {
                        PrimaryInfo {}
                    } else if tab_to_show() == "security".to_string() {
                        Security {}
                    }
                }
            }
        }
    }
}

#[component]
fn PrimaryInfo() -> Element {
    //
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut bio = use_signal(|| "".to_string());
    let err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    rsx! {
        div { class: "mt-8 space-y-6 w-[600px]",
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Username" }
                input {
                    class: "w-full",
                    r#type: "text",
                    placeholder: "Username",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { username.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Email" }
                input {
                    class: "w-full rounded-md py-2.5",
                    r#type: "text",
                    placeholder: "Email",
                    value: "",
                    maxlength: 64,
                    oninput: move |evt| { email.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Biography" }
                textarea {
                    class: "w-full rounded-md py-2.5 px-3",
                    cols: 64,
                    rows: 6,
                    placeholder: "Biography",
                    value: "",
                    maxlength: 1024,
                    oninput: move |evt| { bio.set(evt.value()) }
                }
            }
            div { class: "text-center my-8",
                button {
                    class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                    onclick: move |_| { async move {} },
                    "Save"
                }
            }
            if err().is_some() {
                div { class: "text-center text-red-600 my-8",
                    span { {err().unwrap()} }
                }
            } else if saved() {
                div { class: "text-center text-green-600 my-8",
                    span { { "Successfully saved" } }
                }
            }
        }
    }
}

#[component]
fn Security() -> Element {
    //
    let mut new_password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());

    rsx! {
        div { class: "mt-8 space-y-6 w-[600px]",
            div { class: "flex flex-row text-sm text-gray-500", { "Id: 123" } }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "New password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Set a new password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { new_password.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Confirm password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Set a new password again",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { confirm_password.set(evt.value()) }
                }
            }
        }
    }
}
