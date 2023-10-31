use std::collections::HashMap;

use gloo_storage::{LocalStorage, Storage};

pub(crate) use schema::*;

use crate::agents::{AgentConfig, AgentName};
use crate::APP_NAME;
use crate::chat::Chat;
use crate::utils::auth::Auth;
use crate::utils::customization::Customization;
use crate::utils::settings::{GPTService, OpenAIModel};

pub(crate) mod schema;
pub(crate) mod conversion;

#[derive(Clone, Debug, PartialEq)]
pub struct StoredStates {
    pub run_count: usize,
    pub customization: Customization,
    pub name_to_configs: HashMap<AgentName, AgentConfig>,
    pub chats: Vec<Chat>,
    pub auth: Option<Auth>,
    pub selected_service: Option<GPTService>,
    pub openai_model: Option<OpenAIModel>,
}


impl StoredStates {
    pub fn get_or_init() -> Self {
        let key = APP_NAME;

        match LocalStorage::get::<RawStoredStates>(key) {
            Ok(value) => value.into(),
            Err(e) => {
                log::error!("error: {}", e);
                let (mut default_chat, name_to_configs) = Chat::default_chat_and_configs();
                default_chat.topic = "Default Chat".to_string();
                let stored_states = Self {
                    run_count: 0,
                    customization: Default::default(),
                    name_to_configs,
                    chats: vec![default_chat],
                    auth: None,
                    selected_service: None,
                    openai_model: None,
                };
                let raw_stored_states = RawStoredStates::from(stored_states);
                if let Err(e) = LocalStorage::set(key, &raw_stored_states) {
                    log::error!("getting local storage error: {}", e);
                }
                raw_stored_states.into()
            }
        }
    }

    pub fn save(&self) {
        let saved_storage: RawStoredStates = self.clone().into();
        match LocalStorage::set(APP_NAME, saved_storage) {
            Ok(_) => log::info!("Saved StoredStates"),
            Err(e) => log::error!("Error when saving StoredStates: {}", e),
        }
    }
}
