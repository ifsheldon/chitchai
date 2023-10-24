use dioxus::prelude::*;
use futures::future::join_all;
use futures_util::StreamExt;
use transprompt::async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs};
use uuid::Uuid;

use crate::agents::AgentId;
use crate::app::{AuthedClient, ChatId, StreamingReply};
use crate::chat::{Chat, ChatManager, LinkedChatHistory, MessageId};
use crate::components::chat::Request;
use crate::utils::{assistant_msg, user_msg};
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
                     chat_manager: &ChatManager) -> Vec<ChatCompletionRequestMessage> {
    chat_msgs
        .iter()
        .map(|msg_id| chat_manager.get(msg_id).unwrap().msg.clone())
        .collect()
}

#[inline]
fn push_history(chat: &mut Chat,
                agent_id: &AgentId,
                msg_id: MessageId) {
    chat
        .agent_histories
        .get_mut(agent_id)
        .unwrap()
        .push(msg_id);
}

async fn post_agent_request(assistant_id: AgentId,
                            user_agent_id: AgentId,
                            chat_idx: usize,
                            authed_client: UseSharedState<AuthedClient>,
                            global: UseSharedState<StoredStates>) {
    let mut global_mut = global.write();
    let chat = &global_mut.chats[chat_idx];
    // get the context to send to AI
    let messages_to_send = map_chat_messages(chat.agent_histories.get(&assistant_id).as_ref().unwrap(), &global_mut.chat_manager);
    // create an empty assistant message for UI to show a message card
    let agent_name = chat.agents.get(&assistant_id).unwrap().name.clone();
    let assistant_reply = assistant_msg("", agent_name);
    let assistant_reply_id = global_mut.chat_manager.insert(assistant_reply);
    // update history, inserting assistant reply that is empty for now
    let chat = &mut global_mut.chats[chat_idx];
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
            .model("gpt-3.5-turbo-0613")
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
                    .chat_manager
                    .get_mut(&assistant_reply_id)
                    .unwrap();
                assistant_reply_msg.merge_delta(&response.choices[0].delta);
            }
            Err(e) => log::error!("OpenAI Error: {:?}", e),
        }
    }
}


pub(super) async fn handle_request(mut rx: UnboundedReceiver<Request>,
                                   chat_id: UseSharedState<ChatId>,
                                   global: UseSharedState<StoredStates>,
                                   authed_client: UseSharedState<AuthedClient>,
                                   streaming_reply: UseSharedState<StreamingReply>) {
    while let Some(Request(request)) = rx.next().await {
        let chat_id = chat_id.read().0;
        log::warn!("chat id = {}", chat_id);
        if authed_client.read().is_none() {
            // TODO: handle this error and make a toast to notify user
            log::error!("authed_client is None");
            continue;
        }
        log::info!("request_handler {}", request);
        let mut global_mut = global.write();
        let chat_idx = find_chat_idx_by_id(&global_mut.chats, &chat_id);
        let chat = &global_mut.chats[chat_idx];
        let user_agent_ids: Vec<AgentId> = chat.user_agent_ids();
        assert_eq!(user_agent_ids.len(), 1, "user_agent_ids.len() == 1"); // TODO: support multiple user agents
        let user_agent_id = user_agent_ids[0];
        let assistant_agent_ids: Vec<AgentId> = chat.assistant_agent_ids();
        // create user message and register them to chat manager
        let user_query = user_msg(request.as_str(), None::<&str>);
        let user_msg_id = global_mut.chat_manager.insert(user_query.clone());
        // update history, inserting user request
        global_mut
            .chats[chat_idx]
            .agent_histories
            .iter_mut()
            .for_each(|(_, history)| history.push(user_msg_id));
        global_mut.save();
        // drop write lock before await point
        drop(global_mut);
        streaming_reply.write().0 = true;
        join_all(
            assistant_agent_ids
                .into_iter()
                .map(|assistant_id|
                    // TODO: now each assistant has independent history, so they don't know the replies from other assistants. Need to update their histories after streaming is done.
                    post_agent_request(assistant_id, user_agent_id, chat_idx, authed_client.to_owned(), global.to_owned())
                )
        ).await;
        // stage assistant reply into local storage
        global.read().save();
        streaming_reply.write().0 = false;
    }
    log::error!("request_handler exited");
}