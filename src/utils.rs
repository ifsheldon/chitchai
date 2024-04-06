use serde::Deserialize;
use transprompt::async_openai::types::{ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent};
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
        msg: ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: string.into(),
            role: Default::default(),
            name: None,
        }),
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
        msg: ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text(string.into()),
                role: Default::default(),
                name
            }
        ),
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
        msg: ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessage{
                content: Some(string.into()),
                role: Default::default(),
                name,
                tool_calls: None,
                function_call: None,
            }
        ),
        metadata: None,
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AgentInstructions {
    pub name: String,
    pub instructions: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Instructions {
    pub agent_config: Vec<AgentInstructions>,
}
