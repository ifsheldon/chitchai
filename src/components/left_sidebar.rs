use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

pub use agent_profiles::*;
pub use chat_history::*;
pub use icons::*;

use crate::app::{ChatId, StreamingReply};
use crate::chat::Chat;
use crate::utils::storage::StoredStates;

pub mod chat_history;
pub mod icons;
pub mod agent_profiles;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeftSidebarEvent {
    ToggleChatHistory,
    ChangeChat(Uuid),
    NewChat,
    EnterDiscovery,
    EnterAgentProfile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondarySidebar {
    History,
    Profile,
    None,
}

impl SecondarySidebar {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

pub fn LeftSidebar(cx: Scope) -> Element {
    let secondary_sidebar = use_state(cx, || SecondarySidebar::None);
    let showing_chat_id = use_shared_state::<ChatId>(cx).unwrap();
    let streaming_reply = use_shared_state::<StreamingReply>(cx).unwrap();
    let global = use_shared_state::<StoredStates>(cx).unwrap();
    use_coroutine(cx, |rx| event_handler(rx, secondary_sidebar.to_owned(), showing_chat_id.to_owned(), streaming_reply.to_owned(), global.to_owned()));
    render! {
        aside {
            class: "flex",
            IconSidebar {}
            match secondary_sidebar.get() {
                SecondarySidebar::History => rsx! {
                    ChatHistorySidebar {}
                },
                SecondarySidebar::Profile => rsx! {
                    AgentProfiles {}
                },
                SecondarySidebar::None => rsx! {
                    div {}
                }
            }
        }
    }
}


async fn event_handler(mut rx: UnboundedReceiver<LeftSidebarEvent>,
                       secondary_sidebar: UseState<SecondarySidebar>,
                       showing_chat_id: UseSharedState<ChatId>,
                       streaming_reply: UseSharedState<StreamingReply>,
                       global: UseSharedState<StoredStates>) {
    while let Some(event) = rx.next().await {
        match event {
            LeftSidebarEvent::ToggleChatHistory => {
                secondary_sidebar.modify(|s| {
                    match s {
                        SecondarySidebar::History => SecondarySidebar::None,
                        _ => SecondarySidebar::History,
                    }
                })
            }
            LeftSidebarEvent::NewChat => {
                let mut global = global.write();
                let new_chat = Chat::default(&mut global.chat_manager);
                let new_chat_id = new_chat.id;
                global.chats.push(new_chat);
                global.save();
                if !streaming_reply.read().0 {
                    showing_chat_id.write().0 = new_chat_id;
                }
            }
            LeftSidebarEvent::EnterDiscovery => {
                // TODO: implement entering discovery
                log::info!("EnterDiscovery");
            }
            LeftSidebarEvent::EnterAgentProfile => {
                // TODO: implement entering user profile
                log::info!("EnterAgentProfile");
            }
            LeftSidebarEvent::ChangeChat(chat_id) => {
                if (!streaming_reply.read().0) && showing_chat_id.read().0 != chat_id {
                    log::info!("Changing to Chat {}", chat_id);
                    showing_chat_id.write().0 = chat_id;
                }
            }
            _ => log::warn!("Unknown event: {:?}", event),
        }
    }
}

