use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentId, AgentType};
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
    pub agent_histories: HashMap<AgentId, LinkedChatHistory>,
    pub agents: HashMap<AgentId, AgentConfig>,
}

impl Chat {
    pub fn new(topic: String,
               date: DatetimeString,
               agent_histories: HashMap<AgentId, LinkedChatHistory>,
               agents: HashMap<AgentId, AgentConfig>) -> Self {
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
        let assistant_type = AgentType::Assistant {
            instructions: "A helpful assistant".to_string(),
        };
        let assistant = AgentConfig::new(None,
                                         assistant_type.str().to_string(),
                                         assistant_type);
        let user = AgentConfig::new(None,
                                    AgentType::User.str().to_string(),
                                    AgentType::User);
        let agent_histories = HashMap::from([
            (assistant.id, history.clone()),
            (user.id, history),
        ]);
        let agents = HashMap::from([
            (assistant.id, assistant),
            (user.id, user),
        ]);
        Self::new("New Chat".to_string(), Default::default(), agent_histories, agents)
    }

    pub fn user_agent_ids<B: FromIterator<AgentId>>(&self) -> B {
        self
            .agents
            .iter()
            .filter_map(|(id, config)| {
                if config.agent_type == AgentType::User {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn assistant_agent_ids<B: FromIterator<AgentId>>(&self) -> B {
        self
            .agents
            .iter()
            .filter_map(|(id, config)| {
                if let AgentType::Assistant { .. } = config.agent_type {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn agent_ids(&self) -> Vec<AgentId> {
        self.agents.keys().cloned().collect()
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[readonly::make]
pub(crate) struct RawChat {
    #[readonly]
    pub id: Uuid,
    pub topic: String,
    pub date: DatetimeString,
    // to prevent json parse error when parsing AgentId as keys, this is an adhoc fix
    pub agent_histories: HashMap<String, LinkedChatHistory>,
    pub agents: HashMap<String, AgentConfig>,
}

impl From<Chat> for RawChat {
    fn from(value: Chat) -> Self {
        let Chat {
            id,
            topic,
            date,
            agent_histories,
            agents,
        } = value;
        let agent_histories = agent_histories
            .into_iter()
            .map(|(id, history)| (id.into(), history))
            .collect();
        let agents = agents
            .into_iter()
            .map(|(id, config)| (id.into(), config))
            .collect();
        Self {
            id,
            topic,
            date,
            agent_histories,
            agents,
        }
    }
}

impl Into<Chat> for RawChat {
    fn into(self) -> Chat {
        let RawChat {
            id,
            topic,
            date,
            agent_histories,
            agents,
        } = self;
        let agent_histories = agent_histories
            .into_iter()
            .map(|(id, history)| (id.into(), history))
            .collect();
        let agents = agents
            .into_iter()
            .map(|(id, config)| (id.into(), config))
            .collect();
        Chat {
            id,
            topic,
            date,
            agent_histories,
            agents,
        }
    }
}