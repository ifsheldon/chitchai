use dioxus::prelude::*;
use futures_util::StreamExt;
use transprompt::async_openai::Client;
use transprompt::async_openai::config::AzureConfig;

use crate::components::{ChatContainer, ChatSidebar, SettingSidebar};
use crate::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT;
use crate::utils::auth::Auth;
use crate::utils::storage::StoredStates;
use crate::utils::sys_msg;

pub const APP_NAME: &str = "chitchai";

pub type GPTClient = Client<AzureConfig>;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvents {
    ToggleSidebar,
}

pub fn App(cx: Scope) -> Element {
    let mut stored_states = StoredStates::get_or_init();
    stored_states.run_count += 1;
    stored_states.save();
    log::info!("This is your {} time running ChitChai!", stored_states.run_count);
    // configure share states
    use_shared_state_provider(cx, || stored_states);
    use_shared_state_provider(cx, || GPTClient::with_config(Auth::default().into()));
    let global = use_shared_state::<StoredStates>(cx).unwrap();
    // configure local states
    let hide_sidebar = use_state(cx, || false);
    let init_history = vec![sys_msg(ASSISTANT_SYS_PROMPT)];
    // configure event handler
    use_coroutine(cx, |mut rx| {
        let hide_sidebar = hide_sidebar.to_owned();
        async move {
            while let Some(event) = rx.next().await {
                match event {
                    AppEvents::ToggleSidebar => {
                        hide_sidebar.modify(|h| !(*h));
                    }
                    _ => log::warn!("Unknown event: {:?}", event),
                }
            }
        }
    });
    render! {
        div {
            class: "flex h-full w-full",
            ChatSidebar {}
            div {
                class: "flex-grow overflow-auto",
                ChatContainer {
                    history: init_history,
                }
            }
            div {
                class: "w-1/6",
                hidden: *hide_sidebar.get(),
                SettingSidebar  {}
            }
        }
    }
}