use transprompt::async_openai::types::{ChatCompletionRequestMessage, Role};
use transprompt::utils::llm::openai::ChatMsg;

use crate::agents::AgentName;

pub mod customization;
pub mod storage;
pub mod auth;
pub mod settings;
pub mod datetime;

pub(crate) const EMPTY: String = String::new();

pub fn sys_msg(string: impl Into<String>) -> ChatMsg {
    ChatMsg {
        msg: ChatCompletionRequestMessage {
            role: Role::System,
            content: Some(string.into()),
            name: None,
            function_call: None,
        },
        metadata: None,
    }
}

pub fn user_msg(string: impl Into<String>, name: AgentName) -> ChatMsg {
    let name = match name {
        AgentName::Named(name) => Some(name),
        AgentName::UserDefault => None,
        AgentName::AssistantDefault => {
            log::error!("Cannot use AssistantDefault as user name");
            panic!()
        }
    };
    ChatMsg {
        msg: ChatCompletionRequestMessage {
            role: Role::User,
            content: Some(string.into()),
            name,
            function_call: None,
        },
        metadata: None,
    }
}

pub fn assistant_msg(string: impl Into<String>, name: AgentName) -> ChatMsg {
    let name = match name {
        AgentName::Named(name) => Some(name),
        AgentName::AssistantDefault => None,
        AgentName::UserDefault => {
            log::error!("Cannot use UserDefault as assistant name");
            panic!()
        }
    };
    ChatMsg {
        msg: ChatCompletionRequestMessage {
            role: Role::Assistant,
            content: Some(string.into()),
            name,
            function_call: None,
        },
        metadata: None,
    }
}
