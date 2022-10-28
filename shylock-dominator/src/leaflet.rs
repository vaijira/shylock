use wasm_bindgen::prelude::*;

use crate::property_map_page::PropertyMapHandle;

#[wasm_bindgen(inline_js = r#"
import * as L from "leaflet/dist/leaflet-src.esm.js"

export class LeafletMap {
    constructor(apikey) {
        this._apikey = apikey;
        this._normal_icon = L.icon({
            iconUrl:       'images/marker-icon-black.png',
            iconRetinaUrl: 'images/marker-icon-2x-black.png',
            shadowUrl:     'css/images/marker-shadow.png',
            iconSize:    [25, 41],
            iconAnchor:  [12, 41],
            popupAnchor: [1, -34],
            tooltipAnchor: [16, -28],
            shadowSize:  [41, 41]
        });
        this._opportunity_icon = L.icon({
            iconUrl:       'images/marker-icon-red.png',
            iconRetinaUrl: 'images/marker-icon-2x-red.png',
            shadowUrl:     'css/images/marker-shadow.png',
            iconSize:    [25, 41],
            iconAnchor:  [12, 41],
            popupAnchor: [1, -34],
            tooltipAnchor: [16, -28],
            shadowSize:  [41, 41]
        });
    }

    init_map(lat, lng) {
        this._mymap = L.map('mapid', {
            center: [lat, lng],
            zoom: 5,
            zoomDelta: 1
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

    add_marker(title_text, alt_text, lat, lng, is_opportunity, property) {
        var myIcon = this._normal_icon;
        if (is_opportunity) {
            myIcon = this._opportunity_icon;
        }
        var marker = L.marker([lat, lng], {
            alt: alt_text,
            title: title_text,
            icon: myIcon
        }).addTo(this._mymap);

        var map = this._mymap;

        marker.on('click', function(e) {
            property.show_property(alt_text);
        });

        marker.on('dblclick', function(e) {
            map.setView([lat, lng], 15);
        });
    }

    set_view(lat, lng) {
        this._mymap.setView([lat, lng], 15);
    }
}
"#)]
extern "C" {
    pub type LeafletMap;

    #[wasm_bindgen(constructor)]
    pub fn new(apikey: &str) -> LeafletMap;

    #[wasm_bindgen(method)]
    pub fn init_map(this: &LeafletMap, lat: f64, lng: f64);

    #[wasm_bindgen(method)]
    pub fn add_marker(
        this: &LeafletMap,
        title: &str,
        alt: &str,
        lat: f64,
        lng: f64,
        is_opportunity: bool,
        property_map_handle: PropertyMapHandle,
    );

    #[wasm_bindgen(method)]
    pub fn set_view(this: &LeafletMap, lat: f64, lng: f64);
}
