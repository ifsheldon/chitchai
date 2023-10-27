use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentID, AgentName};
use crate::utils::auth::Auth;
use crate::utils::customization::Customization;
use crate::utils::datetime::DatetimeString;
use crate::utils::settings::{GPTService, OpenAIModel};

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

