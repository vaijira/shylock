use std::sync::Arc;

use dominator::{html, Dom};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use crate::feather::render_svg_crosshair_icon;
use crate::global::{
    CELL_CLASS, DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, TABLE_CLASS, TBODY_CLASS, THEAD_CLASS,
};
use crate::vehicle_view::VehicleView;

#[derive(Debug)]
pub struct VehiclePage {
    vehicle_list: MutableVec<Arc<VehicleView>>,
}

impl VehiclePage {
    pub fn new(vehicle_list: MutableVec<Arc<VehicleView>>) -> Arc<Self> {
        Arc::new(VehiclePage { vehicle_list })
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
        vec![html!("table", {
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
        })]
    }
}
