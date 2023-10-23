use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::app::APP_NAME;
use crate::chat::{Chat, ChatManager};
use crate::utils::auth::Auth;
use crate::utils::customization::Customization;
use crate::utils::settings::GPTService;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StoredStates {
    pub run_count: usize,
    pub customization: Customization,
    pub chat_manager: ChatManager,
    pub chats: Vec<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    pub selected_service: Option<GPTService>,
}

impl StoredStates {
    pub fn get_or_init() -> Self {
        get_or_init_local_storage(APP_NAME, || {
            let chat_manager = ChatManager::new();
            let mut default_chat = Chat::default(&chat_manager);
            default_chat.topic = "Default Chat".to_string();
            let mut default_chat2 = Chat::default(&chat_manager);
            default_chat2.topic = "Default Chat 2".to_string();
            Self {
                run_count: 0,
                customization: Default::default(),
                chat_manager,
                chats: vec![default_chat, default_chat2],
                auth: None,
                selected_service: None,
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