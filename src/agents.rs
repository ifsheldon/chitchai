use serde::{Deserialize, Serialize};
use transprompt::prompt::PromptTemplate;
use uuid::Uuid;

use crate::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT_TEMPLATE;

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
pub struct AgentId {
    id: Uuid,
}


impl AgentId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Into<String> for AgentId {
    fn into(self) -> String {
        self.id.to_string()
    }
}

impl From<String> for AgentId {
    fn from(s: String) -> Self {
        let id = Uuid::parse_str(&s).expect("Failed to parse AgentId from String");
        Self {
            id,
        }
    }
}

#[readonly::make]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AgentConfig {
    pub id: AgentId,
    pub name: Option<String>,
    pub description: String,
    pub agent_type: AgentType,
    pub sys_prompt: String,
}

const EMPTY: String = String::new();

impl AgentConfig {
    pub fn new_user(name: Option<impl Into<String>>, description: impl Into<String>) -> Self {
        Self {
            id: AgentId::new(),
            name: name.map(|n| n.into()),
            description: description.into(),
            agent_type: AgentType::User,
            sys_prompt: EMPTY,
        }
    }

    pub fn new_assistant(name: Option<impl Into<String>>,
                         instructions: impl Into<String>,
                         description: impl Into<String>) -> Self {
        let name = name.map(|n| n.into());
        let instructions = instructions.into();
        let sys_prompt = PromptTemplate::new(ASSISTANT_SYS_PROMPT_TEMPLATE)
            .construct_prompt()
            .fill("name_instructions",
                  name
                      .as_ref()
                      .map(|n| format!("Your name is {}", n))
                      .unwrap_or(String::new()))
            .fill("instructions", instructions.clone())
            .complete()
            .expect("Failed to complete sys_prompt");
        Self {
            id: AgentId::new(),
            name: name.map(|n| n.into()),
            description: description.into(),
            agent_type: AgentType::Assistant { instructions },
            sys_prompt,
        }
    }
}