use std::collections::HashMap;

use uuid::Uuid;

use crate::agents::{AgentConfig, AgentID, AgentInstance, AgentName};
use crate::chat::{Chat, MessageID, MessageManager};
use crate::utils::storage::StoredStates;

use super::schema::*;

impl Into<StoredStates> for RawStoredStates {
    fn into(self) -> StoredStates {
        let RawStoredStates {
            run_count,
            customization,
            name_to_configs,
            chats,
            auth,
            selected_service,
            openai_model
        } = self;
        let name_to_configs = name_to_configs.into_iter().map(|(k, v)| (k.into(), v)).collect();
        let chats = chats.into_iter().map(|c| c.into_chat(&name_to_configs)).collect();
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
}

impl From<StoredStates> for RawStoredStates {
    fn from(value: StoredStates) -> Self {
        let StoredStates {
            run_count,
            customization,
            name_to_configs,
            chats,
            auth,
            selected_service,
            openai_model
        } = value;
        let chats = chats.into_iter().map(|c| c.into()).collect();
        let name_to_configs = name_to_configs.into_iter().map(|(k, v)| (k.into(), v)).collect();
        Self {
            run_count,
            customization,
            name_to_configs,
            chats,
            auth,
            selected_service,
            openai_model,
        }
    }
}


impl From<Chat> for RawChat {
    fn from(value: Chat) -> Self {
        let Chat {
            id, message_manager, topic, date, agents
        } = value;
        let agents = agents.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        let messages = message_manager.messages.into_iter().map(|(k, v)| (k.into(), v)).collect();
        Self {
            id,
            messages,
            topic,
            date,
            agents,
        }
    }
}

impl RawChat {
    pub fn into_chat(self, name_to_configs: &HashMap<AgentName, AgentConfig>) -> Chat {
        let RawChat {
            id, messages, topic, date, agents
        } = self;
        let agents = agents
            .into_iter()
            .map(|(k, v)| (k.into(), v.into_agent_instance(name_to_configs)))
            .collect();
        let messages = messages.into_iter().map(|(k, v)| (k.into(), v)).collect();

        Chat {
            id,
            message_manager: MessageManager {
                messages,
            },
            topic,
            date,
            agents,
        }
    }
}


impl Into<RawAgentID> for AgentID {
    fn into(self) -> UUIDKey {
        self.id.to_string()
    }
}

impl From<RawAgentID> for AgentID {
    fn from(s: UUIDKey) -> Self {
        let id = Uuid::parse_str(&s).expect("Failed to parse AgentId from String");
        Self {
            id,
        }
    }
}

impl Into<RawMessageID> for MessageID {
    fn into(self) -> RawMessageID {
        self.0.to_string()
    }
}

impl From<RawMessageID> for MessageID {
    fn from(s: RawMessageID) -> Self {
        let id = Uuid::parse_str(&s).expect("Failed to parse MessageId from String");
        Self(id)
    }
}

impl Into<RawAgentInstance> for AgentInstance {
    fn into(self) -> RawAgentInstance {
        let AgentInstance { id, config, history } = self;
        let AgentConfig { name, .. } = config;
        let history = history.into_iter().map(|id| id.into()).collect();
        RawAgentInstance {
            id,
            name,
            history,
        }
    }
}

impl RawAgentInstance {
    pub fn into_agent_instance(self, name_to_configs: &HashMap<AgentName, AgentConfig>) -> AgentInstance {
        let RawAgentInstance { id, name, history } = self;
        let config = match name_to_configs.get(&name) {
            Some(config) => config,
            None => {
                log::warn!("AgentConfig not found for name: {:?}", name);
                unreachable!("AgentConfig not found for name: {:?}", name);
            }
        };
        let history = history.into_iter().map(|id| id.into()).collect();
        AgentInstance {
            id,
            config: config.clone(),
            history,
        }
    }
}

impl Into<RawAgentName> for AgentName {
    fn into(self) -> RawAgentName {
        match self {
            AgentName::Named(name) => name,
            AgentName::UserDefault => "_USER".to_string(),
            AgentName::AssistantDefault => "_ASSISTANT".to_string(),
        }
    }
}

impl From<RawAgentName> for AgentName {
    fn from(s: RawAgentName) -> Self {
        match s.as_str() {
            "_USER" => Self::UserDefault,
            "_ASSISTANT" => Self::AssistantDefault,
            _ => Self::Named(s),
        }
    }
}
