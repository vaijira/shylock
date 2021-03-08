#![warn(rust_2018_idioms, missing_docs, warnings)]

//! Single page application for show spanish auction information.

#![recursion_limit = "4096"]
mod app;
mod components;
mod global;
mod route;
mod routes;
mod utils;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

/// Main entry point for shylock ui app
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();

    App::<app::App>::new().mount_to_body();
}
