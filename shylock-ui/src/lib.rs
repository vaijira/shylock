#![recursion_limit = "4096"]
mod app;
mod components;
mod route;
mod routes;

use once_cell::sync::OnceCell;
use shylock_data::types::{Asset, Auction};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub static ASSETS: OnceCell<Vec<Asset>> = OnceCell::new();
pub static AUCTIONS: OnceCell<HashMap<String, Auction>> = OnceCell::new();

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());

    App::<app::App>::new().mount_to_body();
}
