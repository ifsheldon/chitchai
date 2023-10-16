#![allow(non_snake_case)]

use dioxus::prelude::*;
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

fn App(cx: Scope) -> Element {
    let history = Vec::from([
        sys_msg(ASSISTANT_SYS_PROMPT),
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
    dioxus_web::launch(App);
}
