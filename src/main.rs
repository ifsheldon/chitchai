#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::Level;

use chitchai::app::App;
use chitchai::utils::storage::StoredStates;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Main,
    // #[route("/announcements")]
    // Announcements,
}

fn Main(cx: Scope) -> Element {
    let mut stored_states = StoredStates::get_or_init();
    stored_states.run_count += 1;
    stored_states.save();
    log::info!("This is your {} time running ChitChai!", stored_states.run_count);
    render! {
        App {
            stored_states: stored_states
        }
    }
}

fn AppRouter(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    dioxus_web::launch(AppRouter);
}
