#![allow(non_snake_case)]

use dioxus_web::Config;
use log::Level;

use chitchai::app::{App, AppProps};
use chitchai::utils::storage::StoredStates;

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    let mut stored_states = StoredStates::get_or_init();
    stored_states.run_count += 1;
    stored_states.save();
    log::info!("This is your {} time running ChitChai!", stored_states.run_count);
    dioxus_web::launch_with_props(App,
                                  AppProps {
                                      stored_states,
                                  },
                                  Config::new())
}
