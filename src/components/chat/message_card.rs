use dioxus::prelude::*;
use transprompt::async_openai::types::Role;
use transprompt::utils::llm::openai::ChatMsg;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct MessageCardProps {
    chat_msg: ChatMsg,
}

pub fn MessageCard(cx: Scope<MessageCardProps>) -> Element {
    let chat_msg = &cx.props.chat_msg;
    let msg = chat_msg.msg.content.as_ref().unwrap();
    match chat_msg.msg.role {
        Role::System => render! {
                div {
                    class: "flex flex-row-reverse items-start p-5",
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
                class: "flex flex-row-reverse items-start p-5",
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
                class: "flex items-start p-5",
                img {
                    class: "mr-2 h-8 w-8 rounded-full",
                    src: "https://dummyimage.com/128x128/363536/ffffff&text=A"
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