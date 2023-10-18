use dioxus::prelude::*;
use transprompt::async_openai::Client;
use transprompt::async_openai::config::AzureConfig;

use crate::components::PromptMessageContainer;
use crate::prompt_engineer::prompt_templates::ASSISTANT_SYS_PROMPT;
use crate::utils::auth::Auth;
use crate::utils::storage::StoredStates;
use crate::utils::sys_msg;

pub const APP_NAME: &str = "chitchai";
const NONE: Option<&str> = None;

pub type GPTClient = Client<AzureConfig>;

pub fn App(cx: Scope) -> Element {
    let mut stored_states = StoredStates::get_or_init();
    stored_states.run_count += 1;
    stored_states.save();
    log::info!("This is your {} time running ChitChai!", stored_states.run_count);
    // configure share states
    use_shared_state_provider(cx, || stored_states);
    use_shared_state_provider(cx, || GPTClient::with_config(Auth::default().into()));
    let global = use_shared_state::<StoredStates>(cx).unwrap();
    let init_history = Vec::from([
        sys_msg(ASSISTANT_SYS_PROMPT),
    ]);
    render! {
        PromptMessageContainer {
            history: init_history,
        }
    }
}