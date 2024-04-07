use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

pub use message_card::*;

use crate::agents::AgentID;
use crate::pages::app::{AuthedClient, ChatId, StreamingReply};
use crate::chat::Chat;
use crate::components::chat::request_utils::{find_chat_idx_by_id, handle_request};
use crate::utils::storage::StoredStates;

mod request_utils;
pub mod message_card;

struct Request(String);


pub fn ChatContainer() -> Element {
    let stored_states = use_context::<Signal<StoredStates>>();
    let authed_client = use_context::<Signal<AuthedClient>>();
    let streaming_reply = use_context::<Signal<StreamingReply>>();
    let chat_id = use_context::<Signal<ChatId>>();
    // request handler
    use_coroutine(|rx|
                      handle_request(rx,
                                     chat_id.to_owned(),
                                     stored_states.to_owned(),
                                     authed_client.to_owned(),
                                     streaming_reply.to_owned()),
    );
    // get data
    let stored_states = stored_states.read();
    let chat_idx = find_chat_idx_by_id(&stored_states.chats, &chat_id.read().0);
    let chat: &Chat = &stored_states.chats[chat_idx];
    let user_agent_id: Vec<AgentID> = chat.user_agent_ids();
    assert_eq!(user_agent_id.len(), 1, "user_agents.len() == 1");  // TODO: support multiple user agents
    let user_agent = chat.agents.get(&user_agent_id[0]).unwrap();
    let history = &user_agent.history;
    rsx! {
        div {
            class: "flex h-full w-full flex-col relative",
            div {
                class: "flex flex-col h-full space-y-6 bg-slate-200 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                div {
                    class: "overflow-auto max-h-[90vh] flex-grow dark:scrollbar dark:scrollbar-thumb-slate-700 dark:scrollbar-track-slate-900",
                    {
                        history
                            .iter()
                            .map(|msg_id| {
                                let msg = chat.message_manager.get(msg_id).unwrap();
                                rsx! {
                                    MessageCard {
                                        chat_msg: msg.clone()
                                    }
                                }
                            })
                    }
                }
                ChatMessageInput {
                    disable_submit: streaming_reply.read().0
                }
            }
        }
    }
}


#[component]
pub fn ChatMessageInput(disable_submit: bool) -> Element {
    // TODO: Test new code after adaptation
    const TEXTAREA_ID: &str = "chat-input";
    let stored_states = use_context::<Signal<StoredStates>>();
    let tick = use_signal(|| 0_usize);
    // configure timer
    use_coroutine(|_: UnboundedReceiver<()>| {
        let mut tick = tick.to_owned();
        async move {
            loop {
                sleep(Duration::from_millis(500)).await;
                tick.with_mut(|tick| *tick = tick.wrapping_add(1));
            }
        }
    });
    let request_sender: Coroutine<Request> = use_coroutine_handle();
    let mut input_value = use_signal(|| String::new());
    // TODO: try not to use js to clear textarea
    let js = format!("document.getElementById('{}').value = '';", TEXTAREA_ID);
    let create_eval = eval(&js);
    let mut clear_textarea = use_resource(move || {
        let create_eval = create_eval.to_owned();
        async move {
            let result = create_eval.join().await;
            match result {
                Ok(_) => log::info!("clear_textarea"),
                Err(e) => log::error!("clear_textarea error: {:?}", e),
            }
        }
    });

    rsx! {
        form {
            class: "mt-2 absolute bottom-0 w-full p-5",
            id: "chat-form",
            onsubmit: move |_| {
                let input_str = input_value.read().clone();
                log::info!("onsubmit {}", input_str);
                request_sender.send(Request(input_str));
                input_value.with_mut(|input_value| input_value.clear());
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
                    oninput: move |event| {
                        let val = event.data.value();
                        #[cfg(debug_assertions)]
                        {
                            log::info!("input_value: {}", val);
                        }
                        input_value.set(val);
                    },
                    id: "chat-input",
                    form: "chat-form",
                    class: "block w-full resize-none rounded-xl border-none bg-slate-200 p-4 pl-10 pr-20 text-sm text-slate-900 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-900 dark:text-slate-200 dark:placeholder-slate-400 dark:focus:ring-blue-600 sm:text-base",
                    placeholder: "Enter your prompt",
                    rows: "2",
                    required: true,
                }
                button {
                    r#type: "submit",
                    disabled: disable_submit,
                    class: "absolute bottom-2 right-2.5 rounded-lg bg-blue-700 px-4 py-2 text-sm font-medium text-slate-200 hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 sm:text-base",
                    {
                        if disable_submit {
                            let stored_states = stored_states.read();
                            stored_states.customization.waiting_icons[*tick.read() % stored_states.customization.waiting_icons.len()].clone()
                        } else {
                            "Send".to_string()
                        }
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
