#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::Level;

use chitchai::pages::*;

#[derive(Routable, Clone)]
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


#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
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
