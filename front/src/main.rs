#![allow(non_snake_case)]
mod components;
mod models;

use components::{Footer, Header, Button, Modal};
use models::CreateSession;
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CreateSession(false));
    let is_modal_visible = use_shared_state::<CreateSession>(cx).unwrap();
    log::info!("hello");
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
                        "Create Session"
                    }
                }
                div {
                    class: "flex flex-col items-center justify-center p-6",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            is_modal_visible.write().0 = true;
                        },
                        "Join Session"
                    }
                }
                Modal {
                    title: "Hello".to_string(),
                    content: "Nothing".to_string(),
                    on_cancel: |args| log::info!("Modal closed:"),
                    on_create: |args| log::info!("Modal confirmed:"),
                }
            }
            Footer {}
        }
    })
}