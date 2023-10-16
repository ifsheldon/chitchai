use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AgentConfig {
    pub name: String,
    pub description: String,
}