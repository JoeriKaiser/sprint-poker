#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    log::info!("hello");
    cx.render(rsx! {
        div {
            "Hello, Sprint Poker :D !"
        }
    })
}