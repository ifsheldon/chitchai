use dioxus::prelude::*;
use futures_util::StreamExt;
use transprompt::async_openai::Client;
use transprompt::async_openai::config::{AzureConfig, OpenAIConfig};
use uuid::Uuid;

use crate::components::{ChatContainer, LeftSidebar, SettingSidebar};
use crate::utils::auth::Auth;
use crate::utils::storage::StoredStates;


// Global states
pub type AuthedClient = Option<Client>;

pub struct ChatId(pub Uuid);

pub struct StreamingReply(pub bool);

pub fn Main() -> Element {
    let mut stored_states = StoredStates::get_or_init();
    stored_states.run_count += 1;
    stored_states.save();
    log::info!("This is your {} time running ChitChai!", stored_states.run_count);
    rsx! {
        App {
            stored_states: stored_states
        }
    }
}


#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvents {
    ToggleSettingsSidebar,
}


#[component]
pub fn App(stored_states: StoredStates) -> Element {
    let last_chat_id = stored_states.chats.last().unwrap().id;
    let authed_client: AuthedClient = stored_states
        .auth
        .as_ref()
        .map(|auth| {
            match auth {
                Auth::OpenAI { .. } => Client::with_config::<OpenAIConfig>(auth.clone().into()),
                Auth::AzureOpenAI { .. } => Client::with_config::<AzureConfig>(auth.clone().into()),
                _ => unreachable!(),
            }
        });
    let hide_settings_sidebar = stored_states.auth.is_some() && stored_states.selected_service.is_some();
    // configure share states
    let _stored_states = use_context_provider(|| Signal::new(stored_states));
    let _authed_client = use_context_provider(|| Signal::new(authed_client));
    let _last_chat_id = use_context_provider(|| Signal::new(ChatId(last_chat_id)));
    let _streaming_reply = use_context_provider(|| Signal::new(StreamingReply(false)));

    // configure local states
    let hide_setting_sidebar = use_signal(|| hide_settings_sidebar);
    // configure event handler
    use_coroutine(|mut rx| {
        let mut hide_setting_sidebar = hide_setting_sidebar.to_owned();
        async move {
            while let Some(event) = rx.next().await {
                match event {
                    AppEvents::ToggleSettingsSidebar => {
                        hide_setting_sidebar.with_mut(|h| *h = !(*h));
                    }
                    _ => log::warn!("Unknown event: {:?}", event),
                }
            }
        }
    });
    rsx! {
        div {
            class: "flex h-full w-full",
            LeftSidebar {}
            div {
                class: "flex-grow overflow-auto",
                ChatContainer {}
            }
            div {
                class: "w-1/6",
                hidden: *hide_setting_sidebar.read(),
                SettingSidebar  {}
            }
        }
    }
}