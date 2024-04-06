use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

pub use agent_profiles::*;
pub use chat_history::*;
pub use icons::*;

use crate::pages::app::{ChatId, StreamingReply};
use crate::chat::Chat;
use crate::utils::storage::StoredStates;

pub mod chat_history;
pub mod icons;
pub mod agent_profiles;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeftSidebarEvent {
    ChangeChat(Uuid),
    NewChat,
    EnableSecondary(SecondarySidebar),
    DisableSecondary(SecondarySidebar),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

pub fn LeftSidebar() -> Element {
    let secondary_sidebar = use_context_provider(|| Signal::new(SecondarySidebar::None));
    let showing_chat_id = use_context::<Signal<ChatId>>();
    let streaming_reply = use_context::<Signal<StreamingReply>>();
    let global = use_context::<Signal<StoredStates>>();
    use_coroutine(|rx| event_handler(rx, secondary_sidebar.to_owned(), showing_chat_id.to_owned(), streaming_reply.to_owned(), global.to_owned()));
    rsx! {
        aside {
            class: "flex",
            IconSidebar {}
            match *secondary_sidebar.read() {
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
                       mut secondary_sidebar: Signal<SecondarySidebar>,
                       mut showing_chat_id: Signal<ChatId>,
                       streaming_reply: Signal<StreamingReply>,
                       mut global: Signal<StoredStates>) {
    while let Some(event) = rx.next().await {
        match event {
            LeftSidebarEvent::EnableSecondary(secondary) => {
                *secondary_sidebar.write() = secondary;
            }
            LeftSidebarEvent::DisableSecondary(secondary) => {
                if *secondary_sidebar.read() == secondary {
                    let mut secondary_sidebar = secondary_sidebar.write();
                    if *secondary_sidebar == secondary {
                        *secondary_sidebar = SecondarySidebar::None;
                    }
                }
            }
            LeftSidebarEvent::NewChat => {
                // if not streaming, create a new chat
                if !streaming_reply.read().0 {
                    // if secondary sidebar is not history, change it to history
                    if *secondary_sidebar.read() != SecondarySidebar::History {
                        let mut secondary_sidebar = secondary_sidebar.write();
                        if *secondary_sidebar != SecondarySidebar::History {
                            *secondary_sidebar = SecondarySidebar::History;
                        }
                    }
                    let mut global = global.write();
                    let new_chat = Chat::default();
                    let new_chat_id = new_chat.id;
                    global.chats.push(new_chat);
                    global.save();
                    showing_chat_id.write().0 = new_chat_id;
                }
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

