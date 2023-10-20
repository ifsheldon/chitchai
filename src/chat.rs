use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentType};
use crate::utils::datetime::DatetimeString;

pub const DEFAULT_AGENT_TO_DISPLAY: &str = AgentType::User.str();

pub type LinkedChatHistory = Vec<MessageId>;

#[derive(Clone, Copy, Hash, PartialEq, Debug, Eq, Serialize, Deserialize)]
pub struct MessageId(Uuid);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatManager {
    messages: HashMap<MessageId, ChatMsg>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub topic: String,
    pub date: DatetimeString,
    pub agent_histories: HashMap<String, LinkedChatHistory>,
    pub agents: HashMap<String, AgentConfig>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub fn insert(&mut self, msg: ChatMsg) -> MessageId {
        let id = MessageId(Uuid::new_v4());
        self.messages.insert(id.clone(), msg);
        id
    }

    pub fn remove(&mut self, id: &MessageId) -> Option<ChatMsg> {
        self.messages.remove(id)
    }

    pub fn get(&self, id: &MessageId) -> Option<&ChatMsg> {
        self.messages.get(id)
    }

    pub fn get_mut(&mut self, id: &MessageId) -> Option<&mut ChatMsg> {
        self.messages.get_mut(id)
    }

    pub fn update(&mut self, id: &MessageId, msg: ChatMsg) -> Option<ChatMsg> {
        self.messages.insert(id.clone(), msg)
    }
}