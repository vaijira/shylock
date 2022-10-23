use std::sync::Arc;

use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shylock_data::{BidInfo, Other};

use crate::{
    feather::render_svg_external_link_icon,
    global::{
        AUCTIONS, CELL_CLASS, CELL_EXPANDED_CLASS, CELL_FLEX_CONTAINER_CLASS, CELL_FLEX_ITEM_CLASS,
        DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, ROW_CLASS,
    },
    util::{format_valuation, is_targeted_asset, new_bidinfo, summarize, DESCRIPTION_TEXT_LIMIT},
};

#[derive(Debug)]
pub struct OtherAssetView {
    pub anchor_hovered: Mutable<bool>,
    pub show_expanded: Mutable<bool>,
    pub filtered_in: Mutable<bool>,
    pub other: &'static Other,
    pub bidinfo: BidInfo,
}

impl OtherAssetView {
    pub fn new(other: &'static Other) -> Arc<Self> {
        let auction_bidinfo = &AUCTIONS
            .get()
            .unwrap()
            .get(&other.auction_id)
            .unwrap()
            .bidinfo;

        let bidinfo = if other.bidinfo.is_none() {
            auction_bidinfo
        } else {
            other.bidinfo.as_ref().unwrap()
        };
        Arc::new(Self {
            anchor_hovered: Mutable::new(false),
            show_expanded: Mutable::new(false),
            filtered_in: Mutable::new(true),
            other,
            bidinfo: new_bidinfo(bidinfo, auction_bidinfo),
        })
    }

    fn render_expanded(view: Arc<Self>, bidinfo: &BidInfo) -> Dom {
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
                        .attr("href", &format!("https://subastas.boe.es/detalleSubasta.php?idSub={}",&view.other.auction_id))
                        .attr("target", "_blank")
                        .attr("rel", "external nofollow")
                        .text(&view.other.auction_id)
                        .child(render_svg_external_link_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
                        .event(clone!(view => move |_: events::MouseEnter| {
                            *view.anchor_hovered.lock_mut() = true;
                        }))
                        .event(clone!(view => move |_: events::MouseLeave| {
                            *view.anchor_hovered.lock_mut() = false;
                        }))
                    }))
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Categoría: ")
                    .text(view.other.category.name())
                    .text(".")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Descripción: ")
                    .text(&view.other.description)
                    .text(
                        if view.other.description.ends_with('.') { "" }
                        else {"."}
                    )
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Información adicional: ")
                    .text(&view.other.additional_information)
                    .text(
                        if view.other.additional_information.ends_with('.') { "" }
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
                .text(self.other.category.name())
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(summarize(&self.other.description))
                .text(if self.other.description.len() > DESCRIPTION_TEXT_LIMIT {" ..."} else {""})
            }),
            html!("td", {
                .class(&*CELL_CLASS)
                .text(summarize(&self.other.additional_information))
                .text(if self.other.additional_information.len() > DESCRIPTION_TEXT_LIMIT {" ..."} else {""})
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
            .get(&view.other.auction_id)
            .unwrap()
            .bidinfo;

        let bidinfo = if view.other.bidinfo.is_none() {
            auction_bidinfo
        } else {
            view.other.bidinfo.as_ref().unwrap()
        };

        html!("tr", {
            .visible_signal(view.filtered_in.signal())
            .class(&*ROW_CLASS)
            .event(clone!(view => move |_: events::Click| {
                if *view.anchor_hovered.lock_ref() {
                    return;
                }
                let current_value = *view.show_expanded.lock_ref();
                *view.show_expanded.lock_mut() = !current_value;
            }))
            .children_signal_vec(view.show_expanded.signal()
                .map(clone!(view => move |x|
                    if x {
                        vec![OtherAssetView::render_expanded(view.clone(), bidinfo)]
                    } else {
                        view.render_compacted(bidinfo)
                    }
                )).to_signal_vec())
        })
    }
}
