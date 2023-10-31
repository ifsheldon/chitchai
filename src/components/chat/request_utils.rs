use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use futures::future::join_all;
use futures_util::StreamExt;
use transprompt::async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs};
use uuid::Uuid;

use crate::agents::AgentID;
use crate::pages::app::{AuthedClient, ChatId, StreamingReply};
use crate::chat::{Chat, LinkedChatHistory, MessageID, MessageManager};
use crate::components::chat::Request;
use crate::utils::{assistant_msg, EMPTY, user_msg};
use crate::utils::storage::StoredStates;

pub(super) fn find_chat_idx_by_id(chats: &Vec<Chat>, id: &Uuid) -> usize {
    for (idx, c) in chats.iter().enumerate() {
        if c.id.eq(id) {
            return idx;
        }
    }
    unreachable!("Cannot find a chat, should not be since deleting is not implemented yet")
}


#[inline]
fn map_chat_messages(chat_msgs: &LinkedChatHistory,
                     message_manager: &MessageManager) -> Vec<ChatCompletionRequestMessage> {
    chat_msgs
        .iter()
        .map(|msg_id| message_manager.get(msg_id).unwrap().msg.clone())
        .collect()
}

#[inline]
fn push_history(chat: &mut Chat,
                agent_id: &AgentID,
                msg_id: MessageID) {
    chat
        .agents
        .get_mut(agent_id)
        .unwrap()
        .history
        .push(msg_id)
}

#[inline]
fn linearize_replies(mut replies: Vec<(AgentID, MessageID, usize)>) -> LinkedChatHistory {
    replies.sort_by(|(_, _, ord1), (_, _, ord2)| ord1.cmp(ord2));
    replies
        .into_iter()
        .map(|(_agent_id, msg_id, _)| msg_id)
        .collect()
}

async fn post_agent_request(assistant_id: AgentID,
                            user_agent_id: AgentID,
                            chat_idx: usize,
                            authed_client: UseSharedState<AuthedClient>,
                            order: Arc<Mutex<usize>>,
                            global: UseSharedState<StoredStates>) -> (AgentID, MessageID, usize) {
    let mut global_mut = global.write();
    let chat = &global_mut.chats[chat_idx];
    // get the context to send to AI
    let agent = chat.agents.get(&assistant_id).unwrap();
    let messages_to_send = map_chat_messages(&agent.history, &chat.message_manager);
    let agent_name = agent.get_name();
    // update history, inserting assistant reply that is empty initially
    let chat = &mut global_mut.chats[chat_idx];
    let assistant_reply_id = chat.message_manager.insert(assistant_msg(EMPTY, agent_name));
    push_history(chat, &assistant_id, assistant_reply_id);
    push_history(chat, &user_agent_id, assistant_reply_id);
    // drop write lock before await point
    drop(global_mut);
    // send request, returning a stream
    let mut stream = authed_client
        .read()
        .as_ref()
        .unwrap()
        .chat()
        .create_stream(CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo-0613") // TODO: use model when it's OpenAI Service
            .messages(messages_to_send)
            .build()
            .expect("creating request failed"))
        .await
        .expect("creating stream failed");
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(response) => {
                if response.choices.is_empty() {
                    // azure openai service returns empty response on first call
                    continue;
                }
                let mut global_mut = global.write();
                let assistant_reply_msg = global_mut
                    .chats[chat_idx]
                    .message_manager
                    .get_mut(&assistant_reply_id)
                    .unwrap();
                assistant_reply_msg.merge_delta(&response.choices[0].delta);
            }
            Err(e) => log::error!("OpenAI Error: {:?}", e),
        }
    }
    let mut order = order.lock().unwrap();
    let got_order = *order;
    *order += 1;
    (assistant_id, assistant_reply_id, got_order)
}


pub(super) async fn handle_request(mut rx: UnboundedReceiver<Request>,
                                   chat_id: UseSharedState<ChatId>,
                                   global: UseSharedState<StoredStates>,
                                   authed_client: UseSharedState<AuthedClient>,
                                   streaming_reply: UseSharedState<StreamingReply>) {
    while let Some(Request(request)) = rx.next().await {
        let chat_id = chat_id.read().0;
        log::info!("chat id = {}", chat_id);
        if authed_client.read().is_none() {
            // TODO: handle this error and make a toast to notify user
            log::error!("authed_client is None");
            continue;
        }
        log::info!("request_handler {}", request);
        let mut global_mut = global.write();
        let chat_idx = find_chat_idx_by_id(&global_mut.chats, &chat_id);
        let chat = &global_mut.chats[chat_idx];
        let user_agent_ids: Vec<AgentID> = chat.user_agent_ids();
        assert_eq!(user_agent_ids.len(), 1, "user_agent_ids.len() == 1"); // TODO: support multiple user agents
        let user_agent_id = user_agent_ids[0];
        let user_agent = chat.agents.get(&user_agent_id).unwrap();
        let assistant_agent_ids: Vec<AgentID> = chat.assistant_agent_ids();
        // create user message and register them to chat manager
        let user_query = user_msg(request.as_str(), user_agent.get_name());
        let user_msg_id = global_mut.chats[chat_idx].message_manager.insert(user_query.clone());
        // update history, inserting user request
        global_mut
            .chats[chat_idx]
            .agents
            .iter_mut()
            .for_each(|(_, agent)| agent.history.push(user_msg_id));
        global_mut.save();
        // drop write lock before await point
        drop(global_mut);
        streaming_reply.write().0 = true;
        let order = Arc::new(Mutex::new(0_usize));
        let results = join_all(
            assistant_agent_ids
                .iter()
                .map(|assistant_id| post_agent_request(*assistant_id, user_agent_id, chat_idx, authed_client.to_owned(), order.clone(), global.to_owned()))
        ).await;
        let replies = linearize_replies(results);
        // add replies to history of each assistant
        let mut global_mut = global.write();
        let chat = &mut global_mut.chats[chat_idx];
        assistant_agent_ids
            .iter()
            .for_each(|agent_id| {
                for msg_id in replies.iter() {
                    push_history(chat, agent_id, *msg_id);
                }
            });
        drop(global_mut);
        // stage assistant reply into local storage
        global.read().save();
        streaming_reply.write().0 = false;
    }
    log::error!("request_handler exited");
}