#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use log::Level;
use serde::{Deserialize, Serialize};
use transprompt::async_openai::types::{ChatCompletionRequestMessage, Role};
use transprompt::utils::llm::openai::ChatMsg;

use chitchai::components::PromptMessageContainer;
use chitchai::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT;

const NONE: Option<&str> = None;

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

fn get_or_init_local_storage<T, F>(key: &str, default: F) -> T
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

fn App(cx: Scope) -> Element {
    let run_count = get_or_init_local_storage("chitchai", || 0_usize);
    LocalStorage::set("chitchai", run_count + 1).unwrap();
    let history = Vec::from([
        sys_msg(ASSISTANT_SYS_PROMPT),
        sys_msg(format!("This is your {} time running ChitChai!", run_count)),
        user_msg("Explain quantum computing in simple terms", NONE),
        assistant_msg(
            "Certainly! Quantum computing is a new type of computing that relies on the principles of quantum physics. Traditional computers, like the one you might be using right now, use bits to store and process information. These bits can represent either a 0 or a 1. In contrast, quantum computers use quantum bits, or qubits. Unlike bits, qubits can represent not only a 0 or a 1 but also a superposition of both states simultaneously. This means that a qubit can be in multiple states at once, which allows quantum computers to perform certain calculations much faster and more efficiently",
            NONE,
        ),
        user_msg("What are three great applications of quantum computing?", NONE),
        assistant_msg(
            "Three great applications of quantum computing are: Optimization of complex problems, Drug Discovery and Cryptography.",
            NONE,
        ),
    ]);
    render! {
        PromptMessageContainer {
            history: history,
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    dioxus_web::launch(App);
}
