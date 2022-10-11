use std::sync::Arc;

use dominator::{html, Dom};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use crate::feather::render_svg_crosshair_icon;
use crate::global::{
    CELL_CLASS, DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, TABLE_CLASS, TBODY_CLASS, THEAD_CLASS,
};
use crate::other_asset_view::OtherAssetView;

#[derive(Debug)]
pub struct OtherAssetPage {
    other_list: MutableVec<Arc<OtherAssetView>>,
}

impl OtherAssetPage {
    pub fn new(other_list: MutableVec<Arc<OtherAssetView>>) -> Arc<Self> {
        Arc::new(OtherAssetPage { other_list })
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
                            .text("Categoría")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Descripción")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Información adicional")
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
        vec![html!("table", {
            .class(&*TABLE_CLASS)
            .children(&mut[
                page.render_table_header(),
                html!("tbody", {
                    .class(&*TBODY_CLASS)
                    .children_signal_vec(page.other_list.signal_vec_cloned()
                        .map(OtherAssetView::render)
                    )
                }),
            ])
        })]
    }
}
