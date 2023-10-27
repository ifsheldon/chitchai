use serde::{Deserialize, Serialize};
use transprompt::prompt::PromptTemplate;
use uuid::Uuid;

use crate::chat::{LinkedChatHistory, MessageManager};
use crate::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT_TEMPLATE;
use crate::utils::{EMPTY, sys_msg};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentName {
    UserDefault,
    AssistantDefault,
    Named(String),
}

impl AgentName {
    pub fn assistant(name: Option<impl Into<String>>) -> Self {
        match name {
            Some(name) => Self::Named(name.into()),
            None => Self::AssistantDefault,
        }
    }

    pub fn user(name: Option<impl Into<String>>) -> Self {
        match name {
            Some(name) => Self::Named(name.into()),
            None => Self::UserDefault,
        }
    }
}


#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    User,
    Assistant {
        instructions: String,
    },
}


impl AgentType {
    pub const fn str(&self) -> &'static str {
        match self {
            AgentType::User => "User",
            AgentType::Assistant { .. } => "Assistant",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentID {
    pub(crate) id: Uuid,
}

impl AgentID {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentInstance {
    pub id: AgentID,
    pub config: AgentConfig,
    pub history: LinkedChatHistory,
}

impl AgentInstance {
    pub fn new(config: AgentConfig, history: LinkedChatHistory) -> Self {
        Self {
            id: AgentID::new(),
            config,
            history,
        }
    }

    pub fn get_name(&self) -> AgentName {
        self.config.name.clone()
    }

    pub fn default_assistant(name: AgentName, message_manager: &mut MessageManager) -> Self {
        let config = AgentConfig::new_assistant(name, "You are a helpful assistant.", EMPTY);
        let sys_prompt_id = message_manager.insert(sys_msg(config.simple_sys_prompt()));
        Self::new(config, vec![sys_prompt_id])
    }

    pub fn default_user() -> Self {
        let config = AgentConfig::new_user(AgentName::UserDefault, EMPTY);
        Self::new(config, vec![])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AgentConfig {
    pub name: AgentName,
    pub description: String,
    pub agent_type: AgentType,
}

impl AgentConfig {
    pub fn new_user(name: AgentName, description: impl Into<String>) -> Self {
        Self {
            name,
            description: description.into(),
            agent_type: AgentType::User,
        }
    }

    pub fn new_assistant(name: AgentName,
                         instructions: impl Into<String>,
                         description: impl Into<String>) -> Self {
        let instructions = instructions.into();
        Self {
            name,
            description: description.into(),
            agent_type: AgentType::Assistant { instructions },
        }
    }

    pub fn simple_sys_prompt(&self) -> String {
        match &self.agent_type {
            AgentType::User => EMPTY,
            AgentType::Assistant { instructions } => {
                PromptTemplate::new(ASSISTANT_SYS_PROMPT_TEMPLATE)
                    .construct_prompt()
                    .fill("name_instructions", match &self.name {
                        AgentName::UserDefault => EMPTY,
                        AgentName::AssistantDefault => EMPTY,
                        AgentName::Named(name) => format!("Your name is {}.", name),
                    })
                    .fill("instructions", instructions.clone())
                    .complete()
                    .expect("Failed to complete sys_prompt")
            }
        }
    }
}