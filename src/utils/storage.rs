use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub(crate) use schema::*;

use crate::agents::{AgentConfig, AgentName};
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
        RawStoredStates::get_or_init()
    }

    pub fn save(&self) {
        let saved_storage: RawStoredStates = self.clone().into();
        saved_storage.save();
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct Announcements {
    pub(crate) announcement: Vec<Announcement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct Announcement {
    pub(crate) title: String,
    pub(crate) date: String,
    pub(crate) author: String,
    pub(crate) content: String,
}
