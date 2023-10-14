#![allow(non_snake_case)]

use dioxus::prelude::*;

use chitchai::components::PromptMessageContainer;

fn App(cx: Scope) -> Element {
    render! {
        PromptMessageContainer {}
    }
}

fn main() {
    dioxus_web::launch(App);
}
