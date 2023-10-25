use std::rc::Rc;
use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

pub use message_card::*;

use crate::agents::AgentId;
use crate::app::{AuthedClient, ChatId, StreamingReply};
use crate::chat::Chat;
use crate::components::chat::request_utils::{find_chat_idx_by_id, handle_request};
use crate::utils::storage::StoredStates;

mod request_utils;
pub mod message_card;

struct Request(String);


pub fn ChatContainer(cx: Scope) -> Element {
    let stored_states = use_shared_state::<StoredStates>(cx).unwrap();
    let authed_client = use_shared_state::<AuthedClient>(cx).unwrap();
    let streaming_reply = use_shared_state::<StreamingReply>(cx).unwrap();
    let chat_id = use_shared_state::<ChatId>(cx).unwrap();
    // request handler
    use_coroutine(cx, |rx|
        handle_request(rx,
                       chat_id.to_owned(),
                       stored_states.to_owned(),
                       authed_client.to_owned(),
                       streaming_reply.to_owned()),
    );
    // get data
    let stored_states = stored_states.read();
    let chat_manager = &stored_states.chat_manager;
    let chat_idx = find_chat_idx_by_id(&stored_states.chats, &chat_id.read().0);
    let chat: &Chat = &stored_states.chats[chat_idx];
    let user_agent_id: Vec<AgentId> = chat.user_agent_ids();
    assert_eq!(user_agent_id.len(), 1, "user_agents.len() == 1");  // TODO: support multiple user agents
    let history = chat.agent_histories.get(&user_agent_id[0]).unwrap();

    render! {
        div {
            class: "flex h-[100vh] w-full flex-col relative",
            div {
                class: "flex-1 space-y-6 overflow-y-auto bg-slate-200 p-4 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                history
                .iter()
                .map(|msg_id| {
                    let msg = chat_manager.get(msg_id).unwrap();
                    rsx! {
                        MessageCard {
                            chat_msg: msg.clone()
                        }
                    }
                })
                ChatMessageInput {
                    disable_submit: streaming_reply.read().0
                }
            }
        }
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
            class: "mt-2 absolute bottom-0 w-full pr-10 pb-5",
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
