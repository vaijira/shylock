#![recursion_limit = "4096"]
mod app;
mod components;
mod route;
mod routes;

use once_cell::sync::OnceCell;
use rust_decimal::Decimal;
use shylock_data::provinces::Province;
use shylock_data::types::{Asset, Auction};
use std::collections::{BTreeSet, HashMap};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub static ASSETS: OnceCell<Vec<Asset>> = OnceCell::new();
pub static AUCTIONS: OnceCell<HashMap<String, Auction>> = OnceCell::new();
pub static MAX_AUCTION_VALUE: OnceCell<Decimal> = OnceCell::new();
pub static PROVINCES: OnceCell<BTreeSet<Province>> = OnceCell::new();
pub static CITIES: OnceCell<BTreeSet<&str>> = OnceCell::new();

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());

    App::<app::App>::new().mount_to_body();
}
