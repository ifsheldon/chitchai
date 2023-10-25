use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::app::APP_NAME;
use crate::chat::{Chat, ChatManager, RawChat};
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
        let key = APP_NAME;

        match LocalStorage::get::<RawStoredStates>(key) {
            Ok(value) => value.into(),
            Err(e) => {
                log::error!("error: {}", e);
                let mut chat_manager = ChatManager::new();
                let mut default_chat = Chat::default(&mut chat_manager);
                default_chat.topic = "Default Chat".to_string();
                let mut default_chat2 = Chat::default(&mut chat_manager);
                default_chat2.topic = "Default Chat 2".to_string();
                let stored_states = Self {
                    run_count: 0,
                    customization: Default::default(),
                    chat_manager,
                    chats: vec![default_chat, default_chat2],
                    auth: None,
                    selected_service: None,
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


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct RawStoredStates {
    pub run_count: usize,
    pub customization: Customization,
    pub chat_manager: ChatManager,
    // to prevent json parse error when parsing Chat, this is an adhoc fix
    pub chats: Vec<RawChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    pub selected_service: Option<GPTService>,
}

impl Into<StoredStates> for RawStoredStates {
    fn into(self) -> StoredStates {
        let RawStoredStates {
            run_count,
            customization,
            chat_manager,
            chats,
            auth,
            selected_service,
        } = self;
        StoredStates {
            run_count,
            customization,
            chat_manager,
            chats: chats.into_iter().map(|c| c.into()).collect(),
            auth,
            selected_service,
        }
    }
}

impl From<StoredStates> for RawStoredStates {
    fn from(value: StoredStates) -> Self {
        let StoredStates {
            run_count,
            customization,
            chat_manager,
            chats,
            auth,
            selected_service,
        } = value;
        Self {
            run_count,
            customization,
            chat_manager,
            chats: chats.into_iter().map(|c| c.into()).collect(),
            auth,
            selected_service,
        }
    }
}