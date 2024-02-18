use dioxus::prelude::*;

use crate::components::Button;
use crate::models::{ButtonType, CreateSession};

#[derive(Props)]
pub struct ModalProps<'a> {
    pub title: String,
    pub content: String,
    pub on_cancel: EventHandler<'a, MouseEvent>,
    pub on_create: EventHandler<'a, MouseEvent>,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps>) -> Element<'a> {
    let is_modal_visible = use_shared_state::<CreateSession>(cx).unwrap();

    if !is_modal_visible.read().0 {
        return None;
    }
    cx.render(rsx!(
        div {
            class: "fixed z-50 inset-0 overflow-y-auto",
            div {
                class: "flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                div {
                    class: "fixed inset-0 transition-opacity",
                    "aria-hidden": "true",
                    class: "bg-gray-500 opacity-75",
                }
                span {
                    class: "hidden sm:inline-block sm:align-middle sm:h-screen",
                    "aria-hidden": "true",
                }
                div {
                    class: "inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    div {
                        class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        header {
                            class: "text-lg font-semibold text-gray-900",
                            h2 {
                                class: "text-xl text-teal-950 font-semibold",
                                "🎬 Sessions"
                            }
                        }
                        form {
                            class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                            div {
                                class: "w-full",
                                label {
                                    class: "text-sm font-semibold",
                                    "Poster"
                                }
                                input {
                                    class: "w-full border border-gray-300 rounded-lg p-2",
                                    "type": "text",
                                    placeholder: "Enter film poster URL",
                                }
                            }
                        }
                        footer {
                            class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                            Button {
                                button_type: ButtonType::Secondary,
                                onclick: move |evt| {
                                    cx.props.on_cancel.call(evt)
                                },
                                "Cancel"
                            }
                            // Button {
                            //     button_type: ButtonType::Primary,
                            //     onclick: move |evt| {
                            //         cx.props.on_create_or_update.call(evt);
                            //     },
                            //     "Save film"
                            // }
                        }
                    }
                }
            }
        }
    ))
}


