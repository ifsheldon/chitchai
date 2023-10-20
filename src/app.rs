use dioxus::prelude::*;
use futures_util::StreamExt;
use transprompt::async_openai::Client;
use transprompt::async_openai::config::{AzureConfig, OpenAIConfig};

use crate::components::{ChatContainer, ChatSidebar, SettingSidebar};
use crate::utils::auth::Auth;
use crate::utils::storage::StoredStates;

pub const APP_NAME: &str = "chitchai";

#[derive(Debug, Clone)]
pub enum GPTClient {
    Azure(Client<AzureConfig>),
    OpenAI(Client<OpenAIConfig>),
}

pub type AuthedClient = Option<GPTClient>;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvents {
    ToggleSettingsSidebar,
}

#[derive(Debug, Clone, Props, PartialEq)]
pub struct AppProps {
    pub stored_states: StoredStates,
}

pub fn App(cx: Scope<AppProps>) -> Element {
    let stored_states = cx.props.stored_states.clone();
    let authed_client: AuthedClient = stored_states
        .auth
        .as_ref()
        .map(|auth| {
            match auth {
                Auth::OpenAI { .. } => GPTClient::OpenAI(Client::with_config(auth.clone().into())),
                Auth::AzureOpenAI { .. } => GPTClient::Azure(Client::with_config(auth.clone().into())),
                _ => unreachable!(),
            }
        });
    let hide_settings_sidebar = stored_states.auth.is_some() && stored_states.selected_service.is_some();
    // configure share states
    use_shared_state_provider(cx, || stored_states);
    use_shared_state_provider(cx, || authed_client);
    let global = use_shared_state::<StoredStates>(cx).unwrap();
    // configure local states
    let hide_setting_sidebar = use_state(cx, || hide_settings_sidebar);
    // configure event handler
    use_coroutine(cx, |mut rx| {
        let hide_setting_sidebar = hide_setting_sidebar.to_owned();
        async move {
            while let Some(event) = rx.next().await {
                match event {
                    AppEvents::ToggleSettingsSidebar => {
                        hide_setting_sidebar.modify(|h| !(*h));
                    }
                    _ => log::warn!("Unknown event: {:?}", event),
                }
            }
        }
    });
    let last_chat_idx = global.read().chats.len() - 1;
    render! {
        div {
            class: "flex h-full w-full",
            ChatSidebar {}
            div {
                class: "flex-grow overflow-auto",
                ChatContainer {
                    chat_idx: last_chat_idx,
                }
            }
            div {
                class: "w-1/6",
                hidden: *hide_setting_sidebar.get(),
                SettingSidebar  {}
            }
        }
    }
}