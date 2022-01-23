#![warn(
    rust_2018_idioms,
    missing_docs,
    missing_debug_implementations,
    unused_extern_crates,
    warnings
)]

//! Single page application for show spanish auction information.

#![recursion_limit = "4096"]
mod app;
mod components;
mod global;
mod route;
mod routes;
mod utils;

use load_dotenv::load_dotenv;
use wasm_bindgen::prelude::*;

load_dotenv!();

pub(crate) static THUNDERFOREST_API_KEY: &str = env!("THUNDERFOREST_API_KEY");

#[wasm_bindgen(module = "/static/map.js")]
extern "C" {
    pub unsafe fn show_map(apikey: &str, lat: f64, lon: f64);
}

/// Main entry point for shylock ui app
#[allow(clippy::unused_unit)]
#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();
    yew::start_app::<app::App>();
}
