//! # Components
//!
//! ## Reference:
//! 1. https://www.langui.dev/components/prompt-containers#component-2
//!

use std::rc::Rc;
use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use transprompt::async_openai::types::Role;
use transprompt::utils::llm::openai::ChatMsg;

use crate::app::Tick;
use crate::utils::{assistant_msg, user_msg};
use crate::utils::storage::StoredStates;

struct Request(String);

async fn handle_request(mut rx: UnboundedReceiver<Request>,
                        history: UseRef<Vec<ChatMsg>>,
                        processing_flag: UseState<bool>) {
    while let Some(Request(request)) = rx.next().await {
        log::info!("request_handler {}", request);
        history.with_mut(|h| {
            h.push(user_msg(request.as_str(), None::<&str>));
            h.push(assistant_msg("", None::<&str>));
        });
        processing_flag.set(true);
        for c in request.chars() {
            history.with_mut(|h| {
                h.last_mut().unwrap().msg.content.as_mut().unwrap().push(c);
            });
            sleep(Duration::from_millis(300)).await
        }
        processing_flag.set(false);
    }
    log::error!("request_handler exited");
}

#[inline_props]
pub fn PromptMessageContainer(cx: Scope, history: Vec<ChatMsg>) -> Element {
    let history = use_ref(cx, || history.clone());
    let request_processing = use_state(cx, || false);
    let request_handler = use_coroutine(cx, |mut rx: UnboundedReceiver<Request>|
        handle_request(rx, history.to_owned(), request_processing.to_owned()),
    );
    // TODO: fix top round corners are white when dark mode is enabled
    render! {
        div {
            class: "flex h-[100vh] w-full flex-col",
            div {
                class: "flex-1 space-y-6 overflow-y-auto rounded-xl bg-slate-200 p-4 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                history.read().iter().map(
                    |msg| rsx!{
                        MessageCard {
                            chat_msg: msg.clone()
                        }
                    }
                )
                PromptMessageInput {
                    disable_submit: *request_processing.get()
                }
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

#[inline_props]
pub fn PromptMessageInput(cx: Scope, disable_submit: bool) -> Element {
    const TEXTAREA_ID: &str = "chat-input";
    let customization = &use_shared_state::<StoredStates>(cx).unwrap().read().customization;
    let tick = use_shared_state::<Tick>(cx).unwrap();
    let request_sender: &Coroutine<Request> = use_coroutine_handle(cx).unwrap();
    let input_value = use_state(cx, || {
        let empty_form = FormData {
            value: String::new(),
            values: Default::default(),
            files: None,
        };
        Rc::new(empty_form)
    });
    // TODO: try not to use js to clear textarea
    let create_eval = use_eval(cx);
    let clear_textarea = use_future(cx, (), |_| {
        let create_eval = create_eval.to_owned();
        let clear_js = format!("document.getElementById('{}').value = '';", TEXTAREA_ID);
        async move {
            let result = create_eval(clear_js.as_str())
                .unwrap()
                .join()
                .await;
            match result {
                Ok(_) => log::info!("clear_textarea"),
                Err(e) => log::error!("clear_textarea error: {:?}", e),
            }
        }
    });

    render! {
        form {
            class: "mt-2",
            id: "chat-form",
            onsubmit: move |_| {
                log::info!("onsubmit {}", &input_value.get().value);
                request_sender.send(Request(input_value.get().value.clone()));
                clear_textarea.restart();
            },
            label {
                r#for: "{TEXTAREA_ID}",
                class: "sr-only",
                "Enter your prompt"
            }
            div {
                class: "relative",
                textarea {
                    oninput: move |event| input_value.set(event.data),
                    id: "chat-input",
                    form: "chat-form",
                    class: "block w-full resize-none rounded-xl border-none bg-slate-200 p-4 pl-10 pr-20 text-sm text-slate-900 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-900 dark:text-slate-200 dark:placeholder-slate-400 dark:focus:ring-blue-600 sm:text-base",
                    placeholder: "Enter your prompt",
                    rows: "2",
                    required: true,
                }
                button {
                    r#type: "submit",
                    disabled: *disable_submit,
                    class: "absolute bottom-2 right-2.5 rounded-lg bg-blue-700 px-4 py-2 text-sm font-medium text-slate-200 hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 sm:text-base",
                    if *disable_submit {
                        customization.waiting_icons[tick.read().0 % customization.waiting_icons.len()].as_str()
                    } else {
                        "Send"
                    }
                    span {
                        class: "sr-only",
                        "Send message"
                    }
                }
            }
        }
    }
}



