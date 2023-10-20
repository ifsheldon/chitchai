use std::collections::HashMap;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::agents::AgentConfig;
use crate::agents::AgentType::{Assistant, User};
use crate::app::APP_NAME;
use crate::chat::{Chat, ChatManager};
use crate::utils::customization::Customization;
use crate::utils::sys_msg;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StoredStates {
    pub run_count: usize,
    pub customization: Customization,
    pub chat_manager: ChatManager,
    pub chats: Vec<Chat>,
}

impl StoredStates {
    pub fn get_or_init() -> Self {
        get_or_init_local_storage(APP_NAME, || {
            let mut chat_manager = ChatManager::new();
            let sys_msg_id = chat_manager.insert(sys_msg("You are a helpful assistant"));
            let history = vec![sys_msg_id];
            let assistant = AgentConfig {
                name: Assistant.str().to_string(),
                description: Assistant.str().to_string(),
                agent_type: Assistant,
            };
            let user = AgentConfig {
                name: User.str().to_string(),
                description: User.str().to_string(),
                agent_type: User,
            };
            let agent_histories = HashMap::from([
                (assistant.name.clone(), history.clone()),
                (user.name.clone(), history.clone()),
            ]);
            let agents = HashMap::from([
                (assistant.name.clone(), assistant),
                (user.name.clone(), user),
            ]);
            let default_chat = Chat {
                topic: "Default".to_string(),
                date: Default::default(),
                agent_histories,
                agents,
            };
            Self {
                run_count: 0,
                customization: Default::default(),
                chat_manager,
                chats: vec![default_chat],
            }
        })
    }

    pub fn save(&self) {
        match LocalStorage::set(APP_NAME, self) {
            Ok(_) => log::info!("Saved StoredStates"),
            Err(e) => log::error!("Error when saving StoredStates: {}", e),
        }
    }
}

fn get_or_init_local_storage<T, F>(key: &str, default: F) -> T
    where T: for<'de> Deserialize<'de> + Serialize + Clone, F: FnOnce() -> T
{
    match LocalStorage::get::<T>(key) {
        Ok(value) => value,
        Err(e) => {
            log::error!("error: {}", e);
            let default = default();
            LocalStorage::set(key, default.clone()).unwrap();
            default
        }
    }
}