use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
}

impl AgentConfig {
    pub fn new(name: Option<String>, description: String, agent_type: AgentType) -> Self {
        Self {
            id: AgentId::new(),
            name,
            description,
            agent_type,
        }
    }
}