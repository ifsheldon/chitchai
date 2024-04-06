use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use transprompt::async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageContent};
use transprompt::utils::llm::openai::ChatMsg;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct MessageCardProps {
    chat_msg: ChatMsg,
}

pub fn MessageCard(cx: Scope<MessageCardProps>) -> Element {
    let chat_msg = &cx.props.chat_msg;
    match &chat_msg.msg {
        ChatCompletionRequestMessage::System(sys_msg) => {
            let content = sys_msg.content.as_str();
            render! {
                div {
                    class: "flex flex-row-reverse items-start p-5",
                    img {
                        class: "ml-2 h-8 w-8 rounded-full",
                        src: "https://dummyimage.com/128x128/354ea1/ffffff&text=S"
                    }
                    MarkdownTextBox {
                        content: content,
                    }
                }
            }
        }
        ChatCompletionRequestMessage::User(user_msg) => {
            let content = match &user_msg.content {
                ChatCompletionRequestUserMessageContent::Text(text) => text.as_str(),
                ChatCompletionRequestUserMessageContent::Array(_) => todo!()
            };
            let name_char = user_msg.name.as_ref().map(|name| name.as_str().chars().next().unwrap()).unwrap_or('U');
            render! {
                div {
                    class: "flex flex-row-reverse items-start p-5",
                    img {
                        class: "ml-2 h-8 w-8 rounded-full",
                        src: "https://dummyimage.com/128x128/354ea1/ffffff&text={name_char}"
                    }
                    MarkdownTextBox {
                        content: content,
                    }
                }
            }
        }
        ChatCompletionRequestMessage::Assistant(assistant_msg) => {
            let content = assistant_msg.content
                .as_ref()
                .expect("Assistant message content is missing; Should not happen as of now")
                .as_str();
            let name_char = assistant_msg.name.as_ref()
                .map(|name| name.as_str().chars().next().unwrap())
                .unwrap_or('A');
            render! {
                div {
                    class: "flex items-start p-5",
                    img {
                        class: "mr-2 h-8 w-8 rounded-full",
                        src: "https://dummyimage.com/128x128/363536/ffffff&text={name_char}"
                    }
                    MarkdownTextBox {
                        content: content,
                    }
                }
            }
        }
        ChatCompletionRequestMessage::Tool(_) | ChatCompletionRequestMessage::Function(_) => todo!(),
    }
}

#[derive(Props, PartialEq, Clone, Debug)]
pub struct MarkdownTextBoxProps<'a> {
    content: &'a str,
}

pub fn MarkdownTextBox<'a>(cx: Scope<'a, MarkdownTextBoxProps>) -> Element<'a> {
    let msg = cx.props.content;
    render! {
        div {
            class: "flex min-h-[85px] rounded-b-xl rounded-tl-xl bg-slate-50 px-4 dark:bg-slate-800 sm:min-h-0 sm:max-w-md md:max-w-2xl",
            article {
                class: "prose dark:prose-invert lg:prose-xl max-w-none",
                    Markdown {
                    content: "{msg}",
                }
            }
        }
    }
}