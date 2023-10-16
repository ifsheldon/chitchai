//! # Components
//!
//! ## Reference:
//! 1. https://www.langui.dev/components/prompt-containers#component-2
//!

use dioxus::prelude::*;
use transprompt::async_openai::types::Role;
use transprompt::utils::llm::openai::ChatMsg;

#[inline_props]
pub fn PromptMessageContainer(cx: Scope, history: Vec<ChatMsg>) -> Element {
    // TODO: fix top round corners are white when dark mode is enabled
    render! {
        div {
            class: "flex h-[100vh] w-full flex-col",
            div {
                class: "flex-1 space-y-6 overflow-y-auto rounded-xl bg-slate-200 p-4 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                history.iter().map(
                    |msg| rsx!{
                        MessageCard {
                            chat_msg: msg.clone()
                        }
                    }
                )
                PromptMessageInput {}
            }
        }
    }
}


#[inline_props]
pub fn MessageCard(cx: Scope, chat_msg: ChatMsg) -> Element {
    let msg = chat_msg.msg.content.as_ref().unwrap();
    match chat_msg.msg.role {
        Role::System => render! {
                div {
                    class: "flex flex-row-reverse items-start",
                    img {
                        class: "ml-2 h-8 w-8 rounded-full",
                        src: "https://dummyimage.com/128x128/354ea1/ffffff&text=S"
                    }
                    div {
                        class: "flex min-h-[85px] rounded-b-xl rounded-tl-xl bg-slate-50 p-4 dark:bg-slate-800 sm:min-h-0 sm:max-w-md md:max-w-2xl",
                        p {
                            "{msg}"
                        }
                    }
                }
            },
        Role::User => render! {
            div {
                class: "flex flex-row-reverse items-start",
                img {
                    class: "ml-2 h-8 w-8 rounded-full",
                    src: "https://dummyimage.com/128x128/354ea1/ffffff&text=U"
                }
                div {
                    class: "flex min-h-[85px] rounded-b-xl rounded-tl-xl bg-slate-50 p-4 dark:bg-slate-800 sm:min-h-0 sm:max-w-md md:max-w-2xl",
                    p {
                        "{msg}"
                    }
                }
            }
        },
        Role::Assistant => render! {
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
        },
        Role::Function => unreachable!(),
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
                textarea {
                    id: "chat-input",
                    class: "block w-full resize-none rounded-xl border-none bg-slate-200 p-4 pl-10 pr-20 text-sm text-slate-900 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-900 dark:text-slate-200 dark:placeholder-slate-400 dark:focus:ring-blue-600 sm:text-base",
                    placeholder: "Enter your prompt",
                    rows: "2",
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



