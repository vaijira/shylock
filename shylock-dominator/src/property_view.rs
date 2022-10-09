use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shylock_data::{BidInfo, Property};
use std::sync::Arc;

use crate::{
    feather::render_svg_external_link_icon,
    global::{
        AUCTIONS, CELL_CLASS, CELL_EXPANDED_CLASS, CELL_FLEX_CONTAINER_CLASS, CELL_FLEX_ITEM_CLASS,
        DEFAULT_ICON_COLOR, ROW_CLASS,
    },
    property_page::PropertyPage,
    util::{format_valuation, is_targeted_asset, summarize, DESCRIPTION_TEXT_LIMIT},
};

#[derive(Debug)]
pub struct PropertyView {
    pub show_expanded: Mutable<bool>,
    pub filtered_in: Mutable<bool>,
    pub property: &'static Property,
}

impl PropertyView {
    pub fn new(property: &'static Property) -> Arc<Self> {
        Arc::new(Self {
            show_expanded: Mutable::new(false),
            filtered_in: Mutable::new(true),
            property,
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
                        .attr("href", &format!("https://subastas.boe.es/detalleSubasta.php?idSub={}",&self.property.auction_id))
                        .attr("target", "_blank")
                        .text(&self.property.auction_id)
                        .child(render_svg_external_link_icon(DEFAULT_ICON_COLOR))
                    }))
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Ciudad: ")
                    .text(&self.property.city)
                    .text(".")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Provincia: ")
                    .text(self.property.province.name())
                    .text(".")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Descripción: ")
                    .text(&self.property.description)
                    .text(
                        if self.property.description.ends_with('.') { "" }
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
                .text(self.property.province.name())
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(&self.property.city)
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(summarize(&self.property.description))
                .text(if self.property.description.len() > DESCRIPTION_TEXT_LIMIT {" ..."} else {""})
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(&format_valuation(&bidinfo.value))
                .text(" €")
            }),
        ]
    }

    pub fn render(page: Arc<PropertyPage>, view: Arc<Self>) -> Dom {
        let auction_bidinfo = &AUCTIONS
            .get()
            .unwrap()
            .get(&view.property.auction_id)
            .unwrap()
            .bidinfo;

        let bidinfo = if view.property.bidinfo.is_none() {
            auction_bidinfo
        } else {
            view.property.bidinfo.as_ref().unwrap()
        };

        html!("tr", {
            .visible_signal(view.filtered_in.signal())
            .class(&*ROW_CLASS)
            .event(clone!(view => move |_: events::Click| {
                let current_value = *view.show_expanded.lock_ref();
                *view.show_expanded.lock_mut() = !current_value;
                if let Some(coordinates) = view.property.coordinates {
                    page.map.set_view(coordinates.lat(), coordinates.lng());
                }
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
