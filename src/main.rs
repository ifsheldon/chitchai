#![allow(non_snake_case)]

use log::Level;

use chitchai::app::App;

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    dioxus_web::launch(App);
}
