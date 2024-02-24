use dioxus::prelude::*;

use crate::components::Button;
use crate::models::{ButtonType, CreateSession};

#[derive(Props)]
pub struct ModalProps<'a> {
    pub title: String,
    pub content: String,
    pub is_visible: bool,
    pub on_cancel: EventHandler<'a, MouseEvent>,
    pub on_create: EventHandler<'a, MouseEvent>,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps>) -> Element<'a> {
    if !cx.props.is_visible {
        return None;
    }
    cx.render(rsx!(
        div {
            class: "fixed inset-0 w-full h-full bg-sky-500/[.06] z-50 max-h-screen max-w-full bg-red-950 flex items-center justify-center",
            div {
                class: "absolute inset-0 bg-gray-500 opacity-75 transition-opacity",
                "aria-hidden": "true",
            }
            span {
                class: "hidden sm:inline-block sm:align-middle",
                "aria-hidden": "true",
            }
            div {
                class: "inline-block align-middle bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:max-w-lg sm:w-full",
                div {
                    class: "px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                    header {
                        class: "text-lg font-semibold text-gray-900",
                        h2 {
                            class: "text-xs font-semibold text-red-950",
                            "{cx.props.title}"
                        }
                    }
                    form {
                        class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                        div {
                            class: "w-full",
                            label {
                                class: "text-sm font-semibold",
                                "{cx.props.content}"
                            }
                            input {
                                class: "w-full border border-gray-300 rounded-lg p-2",
                                "type": "text",
                                placeholder: "Enter the session title",
                            }
                        }
                    }
                    footer {
                        class: "mt-4 flex flex-row justify-center items-center gap-x-2",
                        Button {
                            button_type: ButtonType::Secondary,
                            onclick: move |event| {
                                cx.props.on_cancel.call(event);
                            },
                            "Cancel"
                        }
                    }
                }
            }
        }
    ))
}


