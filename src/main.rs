#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::Level;

use chitchai::pages::app::Main;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Main,
    // #[route("/announcements")]
    // Announcements,
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
