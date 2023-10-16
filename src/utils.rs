use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use transprompt::async_openai::types::{ChatCompletionRequestMessage, Role};
use transprompt::utils::llm::openai::ChatMsg;

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

pub fn user_msg(string: impl Into<String>, name: Option<impl Into<String>>) -> ChatMsg {
    ChatMsg {
        msg: ChatCompletionRequestMessage {
            role: Role::User,
            content: Some(string.into()),
            name: name.map(|n| n.into()),
            function_call: None,
        },
        metadata: None,
    }
}

pub fn assistant_msg(string: impl Into<String>, name: Option<impl Into<String>>) -> ChatMsg {
    ChatMsg {
        msg: ChatCompletionRequestMessage {
            role: Role::Assistant,
            content: Some(string.into()),
            name: name.map(|n| n.into()),
            function_call: None,
        },
        metadata: None,
    }
}

pub fn get_or_init_local_storage<T, F>(key: &str, default: F) -> T
    where T: for<'de> Deserialize<'de> + Serialize + Clone, F: FnOnce() -> T
{
    match LocalStorage::get::<T>(key) {
        Ok(value) => value,
        Err(e) => {
            log::error!("error: {}", e);
            let default = default();
            LocalStorage::set(key, default.clone()).unwrap();
            default
        }
    }
}