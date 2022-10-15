use std::sync::Arc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use rust_decimal::prelude::ToPrimitive;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use crate::feather::render_svg_crosshair_icon;
use crate::global::{
    CAR_BRANDS, CAR_BRAND_MODELS, CELL_CLASS, DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE,
    FILTER_FLEX_CONTAINER_CLASS, TABLE_CLASS, TBODY_CLASS, THEAD_CLASS,
};
use crate::vehicle_view::VehicleView;

const ALL_BRAND_STR: &str = "Todas las marcas";
const ALL_MODEL_STR: &str = "Todos los modelos";

const DEFAULT_OPPORTUNITY_VALUE: f64 = 0.7;

#[derive(Debug)]
pub struct VehiclePage {
    vehicle_list: MutableVec<Arc<VehicleView>>,
    brand_options: MutableVec<&'static str>,
    brand_filter: Mutable<&'static str>,
    opportunity_filter: Mutable<bool>,
    opportunity_filter_threshold: Mutable<f64>,
    model_options: MutableVec<&'static str>,
    model_filter: Mutable<&'static str>,
}

impl VehiclePage {
    pub fn new(vehicle_list: MutableVec<Arc<VehicleView>>) -> Arc<Self> {
        Arc::new(VehiclePage {
            vehicle_list,
            brand_options: MutableVec::new(),
            brand_filter: Mutable::new(ALL_BRAND_STR),
            opportunity_filter: Mutable::new(true),
            opportunity_filter_threshold: Mutable::new(DEFAULT_OPPORTUNITY_VALUE),
            model_options: MutableVec::new(),
            model_filter: Mutable::new(ALL_MODEL_STR),
        })
    }

    fn filter_by_brand(&self, vehicle_view: &Arc<VehicleView>) -> bool {
        let brand = *self.brand_filter.lock_ref();
        if brand == ALL_BRAND_STR {
            true
        } else {
            vehicle_view.vehicle.brand == brand
        }
    }

    fn filter_by_model(&self, vehicle_view: &Arc<VehicleView>) -> bool {
        let model = *self.model_filter.lock_ref();
        if model == ALL_MODEL_STR {
            true
        } else {
            vehicle_view.vehicle.model == model
        }
    }

    fn filter_by_opportunity(&self, vehicle_view: &Arc<VehicleView>) -> bool {
        if *self.opportunity_filter.lock_ref() {
            return true;
        }
        let auction_limit = *self.opportunity_filter_threshold.lock_ref();
        let target_value = vehicle_view.bidinfo.value.to_f64().unwrap_or(0.0) * auction_limit;

        vehicle_view.bidinfo.claim_quantity.to_f64().unwrap_or(0.0) > 1.0
            && target_value > vehicle_view.bidinfo.claim_quantity.to_f64().unwrap_or(0.0)
    }

    fn filter(&self) {
        for vehicle_view in self.vehicle_list.lock_ref().iter() {
            vehicle_view.filtered_in.set_neq(
                self.filter_by_brand(vehicle_view)
                    && self.filter_by_model(vehicle_view)
                    && self.filter_by_opportunity(vehicle_view),
            );
        }
    }

    fn update_model_options(&self) {
        let selected_brand = *self.brand_filter.lock_ref();
        self.model_options.lock_mut().clear();
        self.model_options.lock_mut().push_cloned(ALL_MODEL_STR);
        CAR_BRAND_MODELS
            .get()
            .unwrap()
            .iter()
            .for_each(|(brand, model)| {
                if selected_brand == ALL_BRAND_STR || selected_brand == *brand {
                    self.model_options.lock_mut().push_cloned(model);
                }
            });
        let selected_model = *self.model_filter.lock_ref();
        if !self.model_options.lock_ref().contains(&selected_model) {
            *self.model_filter.lock_mut() = ALL_MODEL_STR;
        }
    }

    fn fill_brand_options(&self) {
        CAR_BRANDS.get().unwrap().iter().for_each(|brand| {
            self.brand_options.lock_mut().push_cloned(brand);
        });
        self.brand_options
            .lock_mut()
            .insert_cloned(0, ALL_BRAND_STR);
        self.update_model_options();
    }

    fn render_filter_section(page: Arc<Self>) -> Dom {
        if page.brand_options.lock_ref().is_empty() {
            page.fill_brand_options();
        }

        html!("div", {
            .class(&*FILTER_FLEX_CONTAINER_CLASS)
            .children(&mut [
            html!("label", {
                .visible(true)
                .attr("for", "checkbox-car-opportunities")
                .text("Oportunidades ")
                .child(render_svg_crosshair_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "checkbox")
                .attr("id", "checkbox-car-opportunities")
                .attr("alt", "Filtrado por oportunidades")
                .with_node!(_input => {
                    .event(clone!(page => move |_: events::Change| {
                        let value = *page.opportunity_filter.lock_ref();
                        *page.opportunity_filter.lock_mut() = !value;
                        page.filter();
                     }))
                })
            }),
            html!("label", {
                .visible(false)
                .attr("for", "select-brand")
                .text("Filtrado por marca:")
            }),
            html!("select" => HtmlSelectElement, {
                .attr("id", "select-brand")
                .attr("alt", "Filtrado por marca")
                .children_signal_vec(page.brand_options.signal_vec_cloned()
                    .map(clone!(page => move |brand|
                        if *page.model_filter.lock_ref() == brand {
                            html!("option", {
                                .attr("selected", "selected")
                                .attr("value", brand)
                                .text(brand)
                            })
                        } else {
                            html!("option", {
                                .attr("value", brand)
                                .text(brand)
                            })
                        })
                    )
                )
                .with_node!(select => {
                    .event(clone!(page => move |_: events::Change| {
                        let lock = page.brand_options.lock_ref();
                        let brand = lock.iter().find(|c|  **c == select.value()).unwrap();
                        *page.brand_filter.lock_mut() = brand;
                        page.update_model_options();
                        page.filter();
                     }))
                })
            }),
            html!("label", {
                .visible(false)
                .attr("for", "select-model")
                .text("Filtrado por modelo:")
            }),
            html!("select" => HtmlSelectElement, {
                .attr("id", "select-model")
                .attr("alt", "Filtrado por modelo")
                .children_signal_vec(page.model_options.signal_vec_cloned()
                    .map(clone!(page => move |model|
                        if *page.model_filter.lock_ref() == model {
                            html!("option", {
                                .attr("selected", "selected")
                                .attr("value", model)
                                .text(model)
                            })
                        } else {
                            html!("option", {
                                .attr("value", model)
                                .text(model)
                            })
                        })
                    )
                )
                .with_node!(select => {
                    .event(clone!(page => move |_: events::Change| {
                        let lock = page.model_options.lock_ref();
                        let model = lock.iter().find(|c|  **c == select.value()).unwrap();
                        *page.model_filter.lock_mut() = model;
                         page.filter();
                     }))
                })
            }),
            ])
        })
    }

    fn render_table_header(&self) -> Dom {
        html!("thead", {
            .class(&*THEAD_CLASS)
            .children(&mut[
                html!("tr", {
                    .style("height", "3em")
                    .children(&mut [
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .child(render_svg_crosshair_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Marca")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Modelo")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Matrícula")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Descripción")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Valor subasta")
                        }),
                    ])
                }),
            ])
        })
    }

    pub fn render(page: Arc<Self>) -> Vec<Dom> {
        vec![
            VehiclePage::render_filter_section(page.clone()),
            html!("table", {
                .class(&*TABLE_CLASS)
                .children(&mut[
                    page.render_table_header(),
                    html!("tbody", {
                        .class(&*TBODY_CLASS)
                        .children_signal_vec(page.vehicle_list.signal_vec_cloned()
                            .map(VehicleView::render)
                        )
                    }),
                ])
            }),
        ]
    }
}