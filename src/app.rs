use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::components::PromptMessageContainer;
use crate::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT;
use crate::utils::{assistant_msg, get_or_init_local_storage, sys_msg, user_msg};

const NONE: Option<&str> = None;

pub fn App(cx: Scope) -> Element {
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