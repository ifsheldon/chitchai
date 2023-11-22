use std::collections::HashMap;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentID, AgentName};
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

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RawChat {
    pub(crate) id: Uuid,
    pub messages: HashMap<RawMessageID, ChatMsg>,
    pub topic: String,
    pub date: DatetimeString,
    pub agents: HashMap<RawAgentID, RawAgentInstance>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RawAgentInstance {
    pub id: AgentID,
    pub name: AgentName,
    pub history: RawLinkedChatHistory,
}

pub(crate) trait StoredState: Serialize + DeserializeOwned {
    const STORE_KEY: &'static str;
    fn get_or_init() -> Self;
    fn save(self) {
        match LocalStorage::set(Self::STORE_KEY, self) {
            Ok(_) => log::info!("Saved StoredState with key {}", Self::STORE_KEY),
            Err(e) => log::error!("Error when saving StoredState with key {}: {}",  Self::STORE_KEY, e),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RawAppSettings {
    pub run_count: usize,
    pub customization: Customization,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_service: Option<GPTService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_model: Option<OpenAIModel>,
}

impl StoredState for RawAppSettings {
    const STORE_KEY: &'static str = "chitchai_settings";

    fn get_or_init() -> Self {
        match LocalStorage::get::<RawAppSettings>(Self::STORE_KEY) {
            Ok(settings) => settings,
            Err(e) => {
                log::error!("error on init RawAppSettings: {}", e);
                let raw_app_settings = RawAppSettings {
                    run_count: 0,
                    customization: Default::default(),
                    auth: None,
                    selected_service: None,
                    openai_model: None,
                };
                raw_app_settings.clone().save();
                raw_app_settings
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RawChats {
    pub chats: Vec<RawChat>,
}

impl StoredState for RawChats {
    const STORE_KEY: &'static str = "chitchai_chats";
    fn get_or_init() -> Self {
        match LocalStorage::get::<RawChats>(Self::STORE_KEY) {
            Ok(value) => value,
            Err(e) => {
                log::error!("error on init RawChats: {}", e);
                let (mut default_chat, _name_to_configs) = Chat::default_chat_and_configs();
                default_chat.topic = "Default Chat".to_string();
                let raw_chats = vec![default_chat.into()];
                let raw_chats = RawChats { chats: raw_chats };
                raw_chats.clone().save();
                raw_chats
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RawAgentConfigs {
    pub name_to_configs: HashMap<RawAgentName, AgentConfig>,
}

impl StoredState for RawAgentConfigs {
    const STORE_KEY: &'static str = "chitchai_agent_configs";
    fn get_or_init() -> Self {
        match LocalStorage::get::<RawAgentConfigs>(Self::STORE_KEY) {
            Ok(configs) => configs,
            Err(e) => {
                log::error!("error on init RawAgentConfigs: {}", e);
                let (_default_chat, name_to_configs) = Chat::default_chat_and_configs();
                let name_to_configs = name_to_configs.into_iter().map(|(k, v)| (k.into(), v)).collect();
                let raw_agent_configs = RawAgentConfigs { name_to_configs };
                raw_agent_configs.clone().save();
                raw_agent_configs
            }
        }
    }
}


pub(crate) struct RawStoredStates {
    pub raw_app_settings: RawAppSettings,
    pub raw_chats: RawChats,
    pub raw_agent_configs: RawAgentConfigs,
}

impl RawStoredStates {
    pub fn get_or_init() -> StoredStates {
        let raw_app_settings = RawAppSettings::get_or_init();
        let raw_chats = RawChats::get_or_init();
        let raw_agent_configs = RawAgentConfigs::get_or_init();
        let name_to_configs = raw_agent_configs.name_to_configs.into_iter().map(|(k, v)| (k.into(), v)).collect();
        let chats = raw_chats.chats.into_iter().map(|c| c.into_chat(&name_to_configs)).collect();
        let RawAppSettings {
            run_count,
            customization,
            auth,
            selected_service,
            openai_model,
        } = raw_app_settings;
        StoredStates {
            run_count,
            customization,
            name_to_configs,
            chats,
            auth,
            selected_service,
            openai_model,
        }
    }

    pub fn save(self) {
        let RawStoredStates {
            raw_app_settings,
            raw_chats,
            raw_agent_configs,
        } = self;
        raw_app_settings.save();
        raw_chats.save();
        raw_agent_configs.save();
    }
}
