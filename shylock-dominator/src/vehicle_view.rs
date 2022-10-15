use std::sync::Arc;

use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use rust_decimal::prelude::ToPrimitive;
use shylock_data::{BidInfo, Vehicle};

use crate::{
    feather::render_svg_external_link_icon,
    global::{
        AUCTIONS, CELL_CLASS, CELL_EXPANDED_CLASS, CELL_FLEX_CONTAINER_CLASS, CELL_FLEX_ITEM_CLASS,
        DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, ROW_CLASS,
    },
    util::{format_valuation, is_targeted_asset, summarize, DESCRIPTION_TEXT_LIMIT},
};

#[derive(Debug)]
pub struct VehicleView {
    pub show_expanded: Mutable<bool>,
    pub filtered_in: Mutable<bool>,
    pub vehicle: &'static Vehicle,
    pub bidinfo: BidInfo,
}

impl VehicleView {
    pub fn new(vehicle: &'static Vehicle) -> Arc<Self> {
        let auction_bidinfo = &AUCTIONS
            .get()
            .unwrap()
            .get(&vehicle.auction_id)
            .unwrap()
            .bidinfo;

        let bidinfo = if vehicle.bidinfo.is_none() {
            auction_bidinfo
        } else {
            vehicle.bidinfo.as_ref().unwrap()
        };

        Arc::new(Self {
            show_expanded: Mutable::new(false),
            filtered_in: Mutable::new(true),
            vehicle,
            bidinfo: BidInfo {
                appraisal: if bidinfo.appraisal.to_f64().unwrap_or(0.0) > 1.0 {
                    bidinfo.appraisal
                } else {
                    auction_bidinfo.appraisal
                },
                bid_step: if bidinfo.bid_step.to_f64().unwrap_or(0.0) > 1.0 {
                    bidinfo.bid_step
                } else {
                    auction_bidinfo.bid_step
                },
                claim_quantity: if bidinfo.claim_quantity.to_f64().unwrap_or(0.0) > 1.0 {
                    bidinfo.claim_quantity
                } else {
                    auction_bidinfo.claim_quantity
                },
                deposit: if bidinfo.deposit.to_f64().unwrap_or(0.0) > 1.0 {
                    bidinfo.deposit
                } else {
                    auction_bidinfo.deposit
                },
                minimum_bid: if bidinfo.minimum_bid.to_f64().unwrap_or(0.0) > 1.0 {
                    bidinfo.minimum_bid
                } else {
                    auction_bidinfo.minimum_bid
                },
                value: if bidinfo.value.to_f64().unwrap_or(0.0) > 1.0 {
                    bidinfo.value
                } else {
                    auction_bidinfo.value
                },
            },
        })
    }

    fn render_expanded(&self, bidinfo: &BidInfo) -> Dom {
        html!("td", {
            .attr("colspan", "5")
            .class(&*CELL_EXPANDED_CLASS)
            .child(html!("div", {
                .class(&*CELL_FLEX_CONTAINER_CLASS)
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Identificador subasta: ")
                    .child(html!("a",{
                        .attr("alt", "Enlace externo a subastas BOE")
                        .attr("href", &format!("https://subastas.boe.es/detalleSubasta.php?idSub={}",&self.vehicle.auction_id))
                        .attr("target", "_blank")
                        .attr("rel", "external nofollow")
                        .text(&self.vehicle.auction_id)
                        .child(render_svg_external_link_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
                    }))
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Marca y modelo: ")
                    .text(&self.vehicle.brand)
                    .text(" ")
                    .text(&self.vehicle.model)
                    .text(".")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Categoría: ")
                    .text(self.vehicle.category.name())
                    .text(".")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Descripción: ")
                    .text(&self.vehicle.description)
                    .text(
                        if self.vehicle.description.ends_with('.') { "" }
                        else {"."}
                    )
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Valor subasta: ")
                    .text(&format_valuation(&bidinfo.value))
                    .text(" €.")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Cantidad reclamada: ")
                    .text(&format_valuation(&bidinfo.claim_quantity))
                    .text(" €.")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Valor tasación: ")
                    .text(&format_valuation(&bidinfo.appraisal))
                    .text(" €.")
                }))
            }))
        })
    }

    fn render_compacted(&self, bidinfo: &BidInfo) -> Vec<Dom> {
        vec![
            html!("td", {
                .class(&*CELL_CLASS)
                .children(
                    &mut is_targeted_asset(bidinfo)[..]
                )
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(&self.vehicle.brand)
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(&self.vehicle.model)
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(&self.vehicle.license_plate)
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(summarize(&self.vehicle.description))
                .text(if self.vehicle.description.len() > DESCRIPTION_TEXT_LIMIT {" ..."} else {""})
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(&format_valuation(&bidinfo.value))
                .text(" €")
            }),
        ]
    }
    pub fn render(view: Arc<Self>) -> Dom {
        let auction_bidinfo = &AUCTIONS
            .get()
            .unwrap()
            .get(&view.vehicle.auction_id)
            .unwrap()
            .bidinfo;

        let bidinfo = if view.vehicle.bidinfo.is_none() {
            auction_bidinfo
        } else {
            view.vehicle.bidinfo.as_ref().unwrap()
        };

        html!("tr", {
            .visible_signal(view.filtered_in.signal())
            .class(&*ROW_CLASS)
            .event(clone!(view => move |_: events::Click| {
                let current_value = *view.show_expanded.lock_ref();
                *view.show_expanded.lock_mut() = !current_value;
            }))
            .children_signal_vec(view.show_expanded.signal()
                .map(clone!(view => move |x|
                    if x {
                        vec![view.render_expanded(bidinfo)]
                    } else {
                        view.render_compacted(bidinfo)
                    }
                )).to_signal_vec())
        })
    }
}
