use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

use crate::components::PromptMessageContainer;
use crate::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT;
use crate::utils::{assistant_msg, sys_msg, user_msg};
use crate::utils::storage::StoredStates;

pub const APP_NAME: &str = "chitchai";
const NONE: Option<&str> = None;

pub struct Tick(pub usize);


pub fn App(cx: Scope) -> Element {
    // configure share states
    let mut stored_states = StoredStates::get_or_init();
    stored_states.run_count += 1;
    stored_states.save();
    use_shared_state_provider(cx, || stored_states);
    let global = use_shared_state::<StoredStates>(cx).unwrap();
    let tick = use_shared_state::<Tick>(cx).unwrap();
    // configure timer
    let timer = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let tick = tick.to_owned();
        async move {
            loop {
                sleep(Duration::from_millis(500)).await;
                let mut tick = tick.write();
                tick.0 = tick.0.wrapping_add(1);
            }
        }
    });
    let history = Vec::from([
        sys_msg(ASSISTANT_SYS_PROMPT),
        sys_msg(format!("This is your {} time running ChitChai!", global.read().run_count)),
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