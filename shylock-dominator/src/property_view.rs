use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shylock_data::{BidInfo, Property};
use std::sync::Arc;

use crate::{
    feather::render_svg_external_link_icon,
    global::{
        AUCTIONS, CELL_CLASS, CELL_EXPANDED_CLASS, CELL_FLEX_CONTAINER_CLASS, CELL_FLEX_ITEM_CLASS,
        DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, ROW_CLASS,
    },
    property_page::PropertyPage,
    util::{
        format_valuation, is_targeted_asset, new_bidinfo, summarize, valid_catastro_reference,
        DESCRIPTION_TEXT_LIMIT,
    },
};

#[derive(Debug)]
pub struct PropertyView {
    pub show_expanded: Mutable<bool>,
    pub filtered_in: Mutable<bool>,
    pub property: &'static Property,
    pub bidinfo: BidInfo,
}

impl PropertyView {
    pub fn new(property: &'static Property) -> Arc<Self> {
        let auction_bidinfo = &AUCTIONS
            .get()
            .unwrap()
            .get(&property.auction_id)
            .unwrap()
            .bidinfo;

        let bidinfo = if property.bidinfo.is_none() {
            auction_bidinfo
        } else {
            property.bidinfo.as_ref().unwrap()
        };

        Arc::new(Self {
            show_expanded: Mutable::new(false),
            filtered_in: Mutable::new(true),
            property,
            bidinfo: new_bidinfo(bidinfo, auction_bidinfo),
        })
    }

    fn render_expanded(&self) -> Dom {
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
                        .attr("href", &format!("https://subastas.boe.es/detalleSubasta.php?idSub={}", &self.property.auction_id))
                        .attr("target", "_blank")
                        .attr("rel", "external nofollow")
                        .text(&self.property.auction_id)
                        .child(render_svg_external_link_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
                    }))
                }))
                .child(if self.property.catastro_link.is_some() {
                    let catastro_link = self.property.catastro_link.clone().unwrap();
                    html!("span", {
                        .class(&*CELL_FLEX_ITEM_CLASS)
                        .text("Referencia catastral: ")
                        .child(html!("a", {
                            .attr("alt", "Enlace externo al catastro")
                            .attr("href", &catastro_link)
                            .attr("target", "_blank")
                            .attr("rel", "external nofollow")
                            .text(&self.property.catastro_reference)
                            .child(render_svg_external_link_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
                        }))
                    }) } else if valid_catastro_reference(&self.property.catastro_reference) {
                        html!("span", {
                            .class(&*CELL_FLEX_ITEM_CLASS)
                            .text("Referencia catastral: ")
                            .text(&self.property.catastro_reference)
                        })
                     }
                        else {
                        Dom::empty()
                    }
                )
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
                    .text(&format_valuation(&self.bidinfo.value))
                    .text(" €.")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Cantidad reclamada: ")
                    .text(&format_valuation(&self.bidinfo.claim_quantity))
                    .text(" €.")
                }))
                .child(html!("span", {
                    .class(&*CELL_FLEX_ITEM_CLASS)
                    .text("Valor tasación: ")
                    .text(&format_valuation(&self.bidinfo.appraisal))
                    .text(" €.")
                }))
            }))
        })
    }

    fn render_compacted(&self) -> Vec<Dom> {
        vec![
            html!("td", {
                .class(&*CELL_CLASS)
                .children(
                    &mut is_targeted_asset(&self.bidinfo)[..]
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
                .text(&format_valuation(&self.bidinfo.value))
                .text(" €")
            }),
        ]
    }

    pub fn render(page: Arc<PropertyPage>, view: Arc<Self>) -> Dom {
        html!("tr", {
            .visible_signal(view.filtered_in.signal())
            .class(&*ROW_CLASS)
            .event(clone!(view => move |_: events::Click| {
                let current_value = *view.show_expanded.lock_ref();
                *view.show_expanded.lock_mut() = !current_value;
                if let Some(coordinates) = view.property.coordinates {
                    page.map.set_view(coordinates.y(), coordinates.x());
                }
            }))
            .children_signal_vec(view.show_expanded.signal()
                .map(clone!(view => move |x|
                    if x {
                        vec![view.render_expanded()]
                    } else {
                        view.render_compacted()
                    }
                )).to_signal_vec())
        })
    }
}
