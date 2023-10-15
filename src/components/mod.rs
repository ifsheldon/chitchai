//! # Components
//!
//! ## Reference:
//! 1. https://www.langui.dev/components/prompt-containers#component-2
//!

use dioxus::prelude::*;

pub fn PromptMessageContainer(cx: Scope) -> Element {
    // TODO: fix top round corners are white when dark mode is enabled
    render! {
        div {
            class: "flex h-[100vh] w-full flex-col",
            div {
                class: "flex-1 space-y-6 overflow-y-auto rounded-xl bg-slate-200 p-4 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                 PromptMessage {
                    msg: "Explain quantum computing in simple terms".to_string(),
                    left: true,
                }
                PromptMessage {
                    msg: "Certainly! Quantum computing is a new type of computing that relies on the principles of quantum physics. Traditional computers, like the one you might be using right now, use bits to store and process information. These bits can represent either a 0 or a 1. In contrast, quantum computers use quantum bits, or qubits. Unlike bits, qubits can represent not only a 0 or a 1 but also a superposition of both states simultaneously. This means that a qubit can be in multiple states at once, which allows quantum computers to perform certain calculations much faster and more efficiently".to_string(),
                    left: false,
                }
                PromptMessage {
                    msg: "What are three great applications of quantum computing?".to_string(),
                    left: true,
                }
                PromptMessage {
                    msg: "Three great applications of quantum computing are: Optimization of complex problems, Drug Discovery and Cryptography.".to_string(),
                    left: false,
                }
                PromptMessageInput {}
            }
        }
    }
}


#[inline_props]
pub fn PromptMessage(cx: Scope, msg: String, left: bool) -> Element {
    if *left {
        render! {
            div {
                class: "flex items-start",
                img {
                    class: "mr-2 h-8 w-8 rounded-full",
                    src: "https://dummyimage.com/128x128/363536/ffffff&text=J"
                }
                div {
                    class: "flex rounded-b-xl rounded-tr-xl bg-slate-50 p-4 dark:bg-slate-800 sm:max-w-md md:max-w-2xl",
                    p {
                        "{msg}"
                    }
                }
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row-reverse items-start",
                img {
                    class: "ml-2 h-8 w-8 rounded-full",
                    src: "https://dummyimage.com/128x128/354ea1/ffffff&text=G"
                }
                div {
                    class: "flex min-h-[85px] rounded-b-xl rounded-tl-xl bg-slate-50 p-4 dark:bg-slate-800 sm:min-h-0 sm:max-w-md md:max-w-2xl",
                    p {
                        "{msg}"
                    }
                }
            }
        }
    }
}


pub fn PromptMessageInput(cx: Scope) -> Element {
    render! {
        form {
            class: "mt-2",
            label {
                r#for: "chat-input",
                class: "sr-only",
                "Enter your prompt"
            }
            div {
                class: "relative",
                button {
                    r#type: "button",
                    class: "absolute inset-y-0 left-0 flex items-center pl-3 text-slate-500 hover:text-blue-600 dark:text-slate-400 dark:hover:text-blue-600",
                    svg {
                        // aria_hidden: "true", FIXME: why aria_hidden is not an attribute in svg?
                        class: "h-5 w-5",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        stroke_width: "2",
                        stroke: "currentColor",
                        fill: "none",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path {
                            stroke: "none",
                            d: "M0 0h24v24H0z",
                            fill: "none"
                        }
                        path {
                            d: "M9 2m0 3a3 3 0 0 1 3 -3h0a3 3 0 0 1 3 3v5a3 3 0 0 1 -3 3h0a3 3 0 0 1 -3 -3z"
                        }
                        path {
                            d: "M5 10a7 7 0 0 0 14 0"
                        }
                        path {
                            d: "M8 21l8 0"
                        }
                        path {
                            d: "M12 17l0 4"
                        }
                    }
                }
                textarea {
                    id: "chat-input",
                    class: "block w-full resize-none rounded-xl border-none bg-slate-200 p-4 pl-10 pr-20 text-sm text-slate-900 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-900 dark:text-slate-200 dark:placeholder-slate-400 dark:focus:ring-blue-600 sm:text-base",
                    placeholder: "Enter your prompt",
                    rows: "1",
                    required: true,
                }
                button {
                    r#type: "submit",
                    class: "absolute bottom-2 right-2.5 rounded-lg bg-blue-700 px-4 py-2 text-sm font-medium text-slate-200 hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 sm:text-base",
                    "Send",
                    span {
                        class: "sr-only",
                        "Send message"
                    }
                }
            }
        }
    }
}



