use std::rc::Rc;
use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use transprompt::async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role};
use transprompt::utils::llm::openai::ChatMsg;

use crate::app::GPTClient;
use crate::chat::{ChatManager, LinkedChatHistory, MessageId};
use crate::utils::{assistant_msg, user_msg};
use crate::utils::storage::StoredStates;

struct Request(String);


#[inline]
fn map_chat_messages(chat_msgs: &LinkedChatHistory,
                     chat_manager: &UseSharedState<ChatManager>) -> Vec<ChatCompletionRequestMessage> {
    let chat_manager = chat_manager.read();
    chat_msgs
        .iter()
        .map(|msg_id| chat_manager.get(msg_id).unwrap().msg.clone())
        .collect()
}


async fn handle_request(mut rx: UnboundedReceiver<Request>,
                        chat_manager_ref: UseSharedState<ChatManager>,
                        history: UseRef<LinkedChatHistory>,
                        gpt_client: UseSharedState<GPTClient>,
                        processing_flag: UseState<bool>) {
    while let Some(Request(request)) = rx.next().await {
        processing_flag.set(true);
        log::info!("request_handler {}", request);
        let mut h = history.write();
        h.push(chat_manager_ref.write().insert(user_msg(request.as_str(), None::<&str>)));
        let request_msgs = map_chat_messages(&h, &chat_manager_ref);
        // push an empty message for UI to show a message card
        h.push(chat_manager_ref.write().insert(assistant_msg("", None::<&str>)));
        drop(h);
        let mut stream = gpt_client.read()
            .chat()
            .create_stream(CreateChatCompletionRequestArgs::default()
                .model("gpt-3.5-turbo-0613")
                .messages(request_msgs)
                .build()
                .expect("creating request failed")
            )
            .await
            .expect("creating stream failed");
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(response) => {
                    if response.choices.is_empty() {
                        // azure openai service returns empty response on first call
                        continue;
                    }
                    let last_msg_id = history.read().last().unwrap().clone();
                    chat_manager_ref
                        .write()
                        .get_mut(&last_msg_id)
                        .unwrap()
                        .merge_delta(&response.choices[0].delta);
                }
                Err(e) => log::error!("OpenAI Error: {:?}", e),
            }
        }
        processing_flag.set(false);
    }
    log::error!("request_handler exited");
}

#[inline_props]
pub fn ChatContainer(cx: Scope, history: Vec<ChatMsg>) -> Element {
    let mut chat_manager = ChatManager::new();
    let history = history.into_iter().map(|msg| {
        let id = chat_manager.insert(msg.clone());
        id
    }).collect::<LinkedChatHistory>();
    use_shared_state_provider(cx, || chat_manager);

    let history = use_ref(cx, || history.clone());
    let request_processing = use_state(cx, || false);

    let gpt_client = use_shared_state::<GPTClient>(cx).unwrap();
    let chat_manager = use_shared_state::<ChatManager>(cx).unwrap();

    // request handler
    use_coroutine(cx, |rx|
        handle_request(rx, chat_manager.to_owned(), history.to_owned(), gpt_client.to_owned(), request_processing.to_owned()),
    );
    render! {
        div {
            class: "flex h-[100vh] w-full flex-col",
            div {
                class: "flex-1 space-y-6 overflow-y-auto bg-slate-200 p-4 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                history.read().iter().map(
                    |msg| rsx!{
                        MessageCard {
                            chat_msg_id: msg.clone()
                        }
                    }
                )
                ChatMessageInput {
                    disable_submit: *request_processing.get()
                }
            }
        }
    }
}


#[inline_props]
pub fn MessageCard(cx: Scope, chat_msg_id: MessageId) -> Element {
    let chat_manager = use_shared_state::<ChatManager>(cx).unwrap().read();
    let chat_msg = chat_manager.get(&chat_msg_id).unwrap();
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

#[inline_props]
pub fn ChatMessageInput(cx: Scope, disable_submit: bool) -> Element {
    const TEXTAREA_ID: &str = "chat-input";
    let customization = &use_shared_state::<StoredStates>(cx).unwrap().read().customization;
    let tick = use_state(cx, || 0_usize);
    // configure timer
    use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let tick = tick.to_owned();
        async move {
            loop {
                sleep(Duration::from_millis(500)).await;
                tick.modify(|tick| tick.wrapping_add(1));
            }
        }
    });
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
                        customization.waiting_icons[*tick.get() % customization.waiting_icons.len()].as_str()
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
