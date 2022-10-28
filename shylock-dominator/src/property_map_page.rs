use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::MutableVec;
use rust_decimal::prelude::ToPrimitive;
use wasm_bindgen::prelude::*;

use crate::leaflet::LeafletMap;
use crate::property_view::PropertyView;
use crate::THUNDERFOREST_API_KEY;

use std::cell::RefCell;
use std::sync::Arc;

#[wasm_bindgen]
#[allow(missing_debug_implementations)]
pub struct PropertyMapHandle {
    handle: Arc<PropertyMapPage>,
    property: Arc<PropertyView>,
}

#[wasm_bindgen]
impl PropertyMapHandle {
    pub fn show_property(&self) {
        PropertyMapPage::show_property(self.handle.clone(), self.property.clone());
    }
}

pub struct PropertyMapPage {
    property_list: MutableVec<Arc<PropertyView>>,
    property_view_present: Mutable<bool>,
    inner_property_view: RefCell<Option<Arc<PropertyView>>>,
    map: LeafletMap,
}

impl PropertyMapPage {
    pub fn new(property_list: MutableVec<Arc<PropertyView>>) -> Arc<Self> {
        Arc::new(PropertyMapPage {
            property_list,
            property_view_present: Mutable::new(false),
            inner_property_view: RefCell::new(None),
            map: LeafletMap::new(THUNDERFOREST_API_KEY),
        })
    }

    pub fn show_property(page: Arc<PropertyMapPage>, property: Arc<PropertyView>) {
        page.inner_property_view.replace(Some(property));
        *page.property_view_present.lock_mut() = true;
    }

    fn is_opportunity(&self, property_view: &PropertyView) -> bool {
        let auction_limit = 0.7;
        let target_value = property_view.bidinfo.value.to_f64().unwrap_or(0.0) * auction_limit;

        property_view.bidinfo.claim_quantity.to_f64().unwrap_or(0.0) > 1.0
            && target_value > property_view.bidinfo.claim_quantity.to_f64().unwrap_or(0.0)
    }

    pub fn render(page: Arc<Self>) -> Vec<Dom> {
        vec![
            html!("div", {
                .attr("id", "mapid")
                .style("z-index", "2147483647")
                .style("height", "400px")
                .style("width", "600px")
                .style("right", "5px")
                .after_inserted(clone!(page => move |_| {
                    page.map.init_map(39.61, -3.69);
                    page.clone().property_list.lock_ref().iter().for_each(|view| {
                        if let Some(coordinates) = view.property.coordinates {
                            let is_opportunity = page.is_opportunity(view);
                            page.map.add_marker(&view.property.auction_id,
                                &view.property.auction_id,
                                coordinates.y(),
                                coordinates.x(),
                                is_opportunity,
                                PropertyMapHandle { handle: page.clone(), property: view.clone() },
                                )

                        }
                    });
                }))
            }),
            html!("div", {
                .style("margin", "5px")
                .child(html!("p", {
                    .text("Información subasta:")
                }))
                .child_signal(page.property_view_present.signal().map(clone!(page => move |present|
                    if present {
                        if let Some(ref view) = *page.inner_property_view.borrow() {
                            Some(PropertyView::render_expanded(view.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                )))
            }),
        ]
    }
    /*
    if let Some(coordinates) = view.property.coordinates {
        page.map.set_view(coordinates.y(), coordinates.x());
    }*/
}
