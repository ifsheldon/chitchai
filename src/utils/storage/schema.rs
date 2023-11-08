use std::collections::HashMap;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentID, AgentName};
use crate::APP_NAME;
use crate::chat::Chat;
use crate::utils::auth::Auth;
use crate::utils::customization::Customization;
use crate::utils::datetime::DatetimeString;
use crate::utils::settings::{GPTService, OpenAIModel};
use crate::utils::storage::StoredStates;

pub(crate) type UUIDKey = String;
pub(crate) type UUIDString = String;

pub(crate) type RawAgentID = UUIDKey;
pub(crate) type RawMessageID = UUIDKey;
pub(crate) type RawLinkedChatHistory = Vec<UUIDString>;
pub(crate) type RawAgentName = String;

#[derive(Serialize, Deserialize)]
pub(crate) struct RawStoredStates {
    pub run_count: usize,
    pub customization: Customization,
    pub name_to_configs: HashMap<RawAgentName, AgentConfig>,
    // to prevent json parse error when parsing Chat, this is an adhoc fix
    pub chats: Vec<RawChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_service: Option<GPTService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_model: Option<OpenAIModel>,
}

impl RawStoredStates {
    fn auth_key() -> String {
        format!("{}-auth", APP_NAME)
    }
    pub fn get_or_init() -> StoredStates {
        match LocalStorage::get::<RawStoredStates>(APP_NAME) {
            Ok(value) => value.into(),
            Err(e) => {
                log::error!("error: {}", e);
                let (mut default_chat, name_to_configs) = Chat::default_chat_and_configs();
                default_chat.topic = "Default Chat".to_string();
                let mut stored_states = StoredStates {
                    run_count: 0,
                    customization: Default::default(),
                    name_to_configs,
                    chats: vec![default_chat],
                    auth: None,
                    selected_service: None,
                    openai_model: None,
                };
                if let Ok(auth) = LocalStorage::get::<Auth>(Self::auth_key()) {
                    stored_states.auth = Some(auth);
                }
                let raw_stored_states = RawStoredStates::from(stored_states.clone());
                raw_stored_states.save();
                stored_states
            }
        }
    }

    pub fn save(self) {
        if self.auth.is_some() {
            match LocalStorage::set(Self::auth_key(), self.auth.clone().unwrap()) {
                Ok(_) => log::info!("Saved Auth"),
                Err(e) => log::error!("Error when saving Auth: {}", e),
            }
        }
        match LocalStorage::set(APP_NAME, self) {
            Ok(_) => log::info!("Saved StoredStates"),
            Err(e) => log::error!("Error when saving StoredStates: {}", e),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RawChat {
    pub(crate) id: Uuid,
    pub messages: HashMap<RawMessageID, ChatMsg>,
    pub topic: String,
    pub date: DatetimeString,
    pub agents: HashMap<RawAgentID, RawAgentInstance>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RawAgentInstance {
    pub id: AgentID,
    pub name: AgentName,
    pub history: RawLinkedChatHistory,
}

