#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::Level;

use chitchai::pages::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Main {},
    #[route("/announcements")]
    AnnouncementPage {},
    #[route("/agents")]
    Agents {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}


#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}


fn AppRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    launch(AppRouter);
}
