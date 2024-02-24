#![allow(non_snake_case)]
mod components;
mod models;

use components::{Footer, Header, Button, Modal};
use models::CreateSession;
use dioxus::prelude::*;
use shared::models::Session;

use crate::models::JoinSession;

const API_ENDPOINT: &str = "api";

fn sessions_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a protocol");
    let protocol = location.protocol().expect("should have a protocol");
    let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
    format!("{}/sessions", endpoint)
}

async fn get_sessions() -> Vec<Session> {
    log::info!("Fetching sessions from {}", sessions_endpoint());
    reqwest::get(&sessions_endpoint())
        .await
        .unwrap()
        .json::<Vec<Session>>()
        .await
        .unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CreateSession(false));
    use_shared_state_provider(cx, || JoinSession(false));
    let is_create_modal_visible = use_shared_state::<CreateSession>(cx).unwrap();
    let is_join_modal_visible = use_shared_state::<JoinSession>(cx).unwrap();
    
    let sessions = use_state::<Option<Vec<Session>>>(cx, || None);

    let force_get_sessions = use_state(cx, || ());

    {
        let sessions = sessions.clone();

        use_effect(cx, force_get_sessions, |_| async move {
            let existing_sessions = get_sessions().await;
            if existing_sessions.is_empty() {
                sessions.set(None);
            } else {
                sessions.set(Some(existing_sessions));
            }
        })
    }

    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                div {
                    class: "flex flex-col items-center justify-center p-6",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            is_create_modal_visible.write().0 = true;
                        },
                        "Create Session"
                    }
                }
                div {
                    class: "flex flex-col items-center justify-center p-6",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            is_join_modal_visible.write().0 = true;
                        },
                        "Join Session"
                    }
                }
                // CREATE
                Modal {
                    title: "Create a session".to_string(),
                    content: "Give the session a name".to_string(),
                    is_visible: is_create_modal_visible.read().0,
                    on_cancel: |_| { is_create_modal_visible.write().0 = false; },
                    on_create: |_| log::info!("CREATE modal confirmed:"),
                }
                // JOIN
                Modal {
                    title: "Join a session".to_string(),
                    content: "Session id you would like to join :".to_string(),
                    is_visible: is_join_modal_visible.read().0,
                    on_cancel: |_| { is_join_modal_visible.write().0 = false; },
                    on_create: |_| log::info!("JOIN modal confirmed:"),
                }
            }
            Footer {}
        }
    })
}