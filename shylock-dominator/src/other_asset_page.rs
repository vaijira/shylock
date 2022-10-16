use std::cmp::Ordering;
use std::sync::Arc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use crate::feather::{
    render_svg_arrow_down_icon, render_svg_arrow_up_icon, render_svg_crosshair_icon,
};
use crate::global::{
    CELL_CLASS, CELL_CLICKABLE_CLASS, DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, TABLE_CLASS,
    TBODY_CLASS, THEAD_CLASS,
};
use crate::other_asset_view::OtherAssetView;
use crate::util::SortingOrder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OtherAssetSorting {
    None,
    ByValue,
    ByReverseValue,
}

#[derive(Debug)]
pub struct OtherAssetPage {
    other_list: MutableVec<Arc<OtherAssetView>>,
    value_sorting: Mutable<SortingOrder>,
    sorting: Mutable<OtherAssetSorting>,
}

impl OtherAssetPage {
    pub fn new(other_list: MutableVec<Arc<OtherAssetView>>) -> Arc<Self> {
        Arc::new(OtherAssetPage {
            other_list,
            value_sorting: Mutable::new(SortingOrder::None),
            sorting: Mutable::new(OtherAssetSorting::None),
        })
    }

    fn sort_by_value(a: &Arc<OtherAssetView>, b: &Arc<OtherAssetView>) -> Ordering {
        a.bidinfo.value.cmp(&b.bidinfo.value)
    }

    fn sort_by_reverse_value(a: &Arc<OtherAssetView>, b: &Arc<OtherAssetView>) -> Ordering {
        b.bidinfo.value.cmp(&a.bidinfo.value)
    }

    fn sort_by_none(_: &Arc<OtherAssetView>, _: &Arc<OtherAssetView>) -> Ordering {
        Ordering::Equal
    }

    fn clear_sortings(&self) {
        *self.value_sorting.lock_mut() = SortingOrder::None;
    }

    fn sorting_by(
        vehicle_sorting: OtherAssetSorting,
    ) -> fn(&Arc<OtherAssetView>, &Arc<OtherAssetView>) -> Ordering {
        match vehicle_sorting {
            OtherAssetSorting::ByValue => OtherAssetPage::sort_by_value,
            OtherAssetSorting::ByReverseValue => OtherAssetPage::sort_by_reverse_value,
            OtherAssetSorting::None => OtherAssetPage::sort_by_none,
        }
    }

    fn render_table_header(page: Arc<Self>) -> Dom {
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
                            .class(&*CELL_CLICKABLE_CLASS)
                            .text("Valor subasta")
                            .child_signal(page.value_sorting.signal().map(|sorting| {
                                match sorting {
                                    SortingOrder::None => Some(Dom::empty()),
                                    SortingOrder::Up => Some(render_svg_arrow_up_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE)),
                                    SortingOrder::Down => Some(render_svg_arrow_down_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE)),
                                }
                            }))
                            .with_node!(_th => {
                                .event(clone!(page => move |_: events::Click| {
                                    let selection = *page.value_sorting.lock_ref();
                                    page.clear_sortings();
                                    match selection {
                                        SortingOrder::None | SortingOrder::Up => {
                                            *page.sorting.lock_mut() = OtherAssetSorting::ByValue;
                                            *page.value_sorting.lock_mut() = SortingOrder::Down;
                                        },
                                        SortingOrder::Down => {
                                            *page.sorting.lock_mut() = OtherAssetSorting::ByReverseValue;
                                            *page.value_sorting.lock_mut() = SortingOrder::Up;
                                        },
                                    }
                                }))
                            })
                        })
                    ])
                }),
            ])
        })
    }

    pub fn render(page: Arc<Self>) -> Vec<Dom> {
        vec![html!("table", {
            .class(&*TABLE_CLASS)
            .children(&mut[
                OtherAssetPage::render_table_header(page.clone()),
                html!("tbody", {
                    .class(&*TBODY_CLASS)
                    .children_signal_vec(
                        page.sorting.signal_ref(|filter| *filter)
                        .switch_signal_vec(clone!(page => move |filter| {
                            page.other_list.signal_vec_cloned()
                            .sort_by_cloned(OtherAssetPage::sorting_by(filter))
                            .map(move |view| {
                                OtherAssetView::render(view)
                            })
                        }))
                    )
                }),
            ])
        })]
    }
}
