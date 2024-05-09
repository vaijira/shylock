#![warn(
    rust_2018_idioms,
    missing_docs,
    missing_debug_implementations,
    unused_extern_crates,
    warnings
)]

//! WASM app to show auction information.

use std::sync::Arc;

use crate::app::App;
use dotenvy_macro::dotenv;
use futures_signals::signal_vec::MutableVec;
use global::{set_global_info, ASSETS};
use other_asset_view::OtherAssetView;
use property_view::PropertyView;
use shylock_data::Asset;
use vehicle_view::VehicleView;
use wasm_bindgen::prelude::*;

mod app;
mod feather;
mod footer;
mod global;
mod leaflet;
mod other_asset_page;
mod other_asset_view;
mod property_map_page;
mod property_page;
mod property_view;
mod route;
mod util;
mod vehicle_page;
mod vehicle_view;

pub(crate) static THUNDERFOREST_API_KEY: &str = dotenv!("THUNDERFOREST_API_KEY");

#[wasm_bindgen(start)]
/// Main point of wasm app entry
pub async fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(
        wasm_logger::Config::new(log::Level::Info).module_prefix(env!("CARGO_PKG_NAME")),
    );

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    set_global_info().await?;

    let properties: MutableVec<Arc<PropertyView>> = MutableVec::new();
    let vehicles: MutableVec<Arc<VehicleView>> = MutableVec::new();
    let other_assets: MutableVec<Arc<OtherAssetView>> = MutableVec::new();

    ASSETS.get().unwrap().iter().for_each(|asset| match asset {
        Asset::Property(property) => properties
            .lock_mut()
            .push_cloned(PropertyView::new(property)),
        Asset::Vehicle(vehicle) => vehicles.lock_mut().push_cloned(VehicleView::new(vehicle)),
        Asset::Other(other) => other_assets
            .lock_mut()
            .push_cloned(OtherAssetView::new(other)),
    });

    let app = App::new(properties, vehicles, other_assets);

    dominator::append_dom(&dominator::get_id("app"), App::render(app));

    Ok(())
}
