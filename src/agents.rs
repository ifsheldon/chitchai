use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AgentConfig {
    pub name: String,
    pub description: String,
    pub agent_type: AgentType,
}