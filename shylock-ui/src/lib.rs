#![recursion_limit = "4096"]
mod app;
mod components;
mod global;
mod route;
mod routes;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();

    App::<app::App>::new().mount_to_body();
}
