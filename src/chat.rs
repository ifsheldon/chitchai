use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentType};
use crate::utils::datetime::DatetimeString;
use crate::utils::sys_msg;

pub const DEFAULT_AGENT_TO_DISPLAY: &str = AgentType::User.str();

pub type LinkedChatHistory = Vec<MessageId>;

#[derive(Clone, Copy, Hash, PartialEq, Debug, Eq, Serialize, Deserialize)]
pub struct MessageId(Uuid);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatManager {
    messages: HashMap<MessageId, ChatMsg>,
    default_sys_prompt_id: MessageId,
}

impl Default for ChatManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[readonly::make]
pub struct Chat {
    #[readonly]
    pub id: Uuid,
    pub topic: String,
    pub date: DatetimeString,
    pub agent_histories: HashMap<String, LinkedChatHistory>,
    pub agents: HashMap<String, AgentConfig>,
}

impl Chat {
    pub fn new(topic: String,
               date: DatetimeString,
               agent_histories: HashMap<String, LinkedChatHistory>,
               agents: HashMap<String, AgentConfig>) -> Self {
        Self {
            id: Uuid::new_v4(),
            topic,
            date,
            agent_histories,
            agents,
        }
    }

    pub fn default(chat_manager: &ChatManager) -> Self {
        let sys_msg_id = chat_manager.default_sys_prompt_id();
        let history = vec![sys_msg_id];
        let assistant = AgentConfig {
            name: AgentType::Assistant.str().to_string(),
            description: AgentType::Assistant.str().to_string(),
            agent_type: AgentType::Assistant,
        };
        let user = AgentConfig {
            name: AgentType::User.str().to_string(),
            description: AgentType::User.str().to_string(),
            agent_type: AgentType::User,
        };
        let agent_histories = HashMap::from([
            (assistant.name.clone(), history.clone()),
            (user.name.clone(), history),
        ]);
        let agents = HashMap::from([
            (assistant.name.clone(), assistant),
            (user.name.clone(), user),
        ]);
        Self::new("New Chat".to_string(), Default::default(), agent_histories, agents)
    }
}

impl Clone for Chat {
    fn clone(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            topic: self.topic.clone(),
            date: self.date.clone(),
            agent_histories: self.agent_histories.clone(),
            agents: self.agents.clone(),
        }
    }
}

impl ChatManager {
    pub fn new() -> Self {
        let default_sys_prompt = sys_msg("You are a helpful assistant");
        let default_sys_prompt_id = MessageId(Uuid::new_v4());
        let messages = HashMap::from([(default_sys_prompt_id, default_sys_prompt)]);
        Self {
            messages,
            default_sys_prompt_id,
        }
    }

    pub fn default_sys_prompt_id(&self) -> MessageId {
        self.default_sys_prompt_id
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