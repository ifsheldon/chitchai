use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Customization {
    pub waiting_icons: Vec<String>,
}

impl Default for Customization {
    fn default() -> Self {
        Self {
            waiting_icons: vec![".".to_string(), "..".to_string(), "...".to_string()]
        }
    }
}