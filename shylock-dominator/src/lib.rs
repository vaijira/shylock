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
mod global;
mod other_asset_page;
mod other_asset_view;
mod property_page;
mod property_view;
mod route;
mod util;
mod vehicle_page;
mod vehicle_view;

pub(crate) static THUNDERFOREST_API_KEY: &str = dotenv!("THUNDERFOREST_API_KEY");

#[wasm_bindgen(inline_js = r#"
export class MyMap {
    constructor(apikey) {
        this._apikey = apikey;
        
    }

    init_map(lat, lng) {
        this._mymap = L.map('mapid', {
            center: [lat, lng],
            zoom: 5,
            zoomDelta: 2
        });

        L.tileLayer('https://tile.thunderforest.com/atlas/{z}/{x}/{y}.png?apikey={accessToken}', {
            attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, Imagery © <a href="https://www.thunderforest.com/">Thunderforest</a>',
            maxZoom: 18,
            id: 'mapbox/streets-v11',
            tileSize: 512,
            zoomOffset: -1,
            accessToken: this._apikey
        }).addTo(this._mymap);
    }

    add_marker(title_text, alt_text, lat, lng) {
        var marker = L.marker([lat, lng], {
            alt: alt_text,
            title: title_text
        }).addTo(this._mymap);

        var map = this._mymap;

        marker.on('click', function(e) {
            map.setView([lat, lng], 15);
        });

        marker.on('dblclick', function(e) {
            window.open('https://subastas.boe.es/detalleSubasta.php?idSub=' + marker.options['title'], '_blank');
        });
    }

    set_view(lat, lng) {
        this._mymap.setView([lat, lng], 15);
    }
}
"#)]
extern "C" {
    pub type MyMap;

    #[wasm_bindgen(constructor)]
    pub fn new(apikey: &str) -> MyMap;

    #[wasm_bindgen(method)]
    pub fn init_map(this: &MyMap, lat: f64, lng: f64);

    #[wasm_bindgen(method)]
    pub fn add_marker(this: &MyMap, title: &str, alt: &str, lat: f64, lng: f64);

    #[wasm_bindgen(method)]
    pub fn set_view(this: &MyMap, lat: f64, lng: f64);
}

#[wasm_bindgen(start)]
/// Main point of wasm app entry
pub async fn main_js() -> Result<(), JsValue> {
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
