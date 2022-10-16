use std::cmp::Ordering;
use std::sync::Arc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use rust_decimal::prelude::ToPrimitive;
use shylock_data::provinces::Province;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use crate::feather::{
    render_svg_arrow_down_icon, render_svg_arrow_up_icon, render_svg_crosshair_icon,
};
use crate::global::{
    CELL_CLASS, CELL_CLICKABLE_CLASS, DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE,
    FILTER_FLEX_CONTAINER_CLASS, TABLE_CLASS, TBODY_CLASS, THEAD_CLASS,
};
use crate::util::SortingOrder;
use crate::{
    global::{CITIES_PROVINCES, PROVINCES},
    property_view::PropertyView,
};
use crate::{MyMap, THUNDERFOREST_API_KEY};

const ALL_CITIES_STR: &str = "Todas las ciudades";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropertySorting {
    None,
    ByProvince,
    ByReverseProvince,
    ByValue,
    ByReverseValue,
}

const DEFAULT_OPPORTUNITY_VALUE: f64 = 0.7;

pub struct PropertyPage {
    property_list: MutableVec<Arc<PropertyView>>,
    city_options: MutableVec<&'static str>,
    city_property_filter: Mutable<&'static str>,
    opportunity_filter: Mutable<bool>,
    opportunity_filter_threshold: Mutable<f64>,
    province_filter: Mutable<Province>,
    province_sorting: Mutable<SortingOrder>,
    value_sorting: Mutable<SortingOrder>,
    sorting: Mutable<PropertySorting>,
    pub map: MyMap,
}

impl PropertyPage {
    pub fn new(property_list: MutableVec<Arc<PropertyView>>) -> Arc<Self> {
        Arc::new(Self {
            property_list,
            city_options: MutableVec::new(),
            city_property_filter: Mutable::new(ALL_CITIES_STR),
            opportunity_filter: Mutable::new(true),
            opportunity_filter_threshold: Mutable::new(DEFAULT_OPPORTUNITY_VALUE),
            province_filter: Mutable::new(Province::All),
            province_sorting: Mutable::new(SortingOrder::None),
            value_sorting: Mutable::new(SortingOrder::None),
            sorting: Mutable::new(PropertySorting::None),
            map: MyMap::new(THUNDERFOREST_API_KEY),
        })
    }

    fn get_province(selected_index: usize) -> Province {
        if selected_index == 0 {
            Province::All
        } else {
            *PROVINCES
                .get()
                .unwrap()
                .iter()
                .enumerate()
                .find(|(i, _)| i == &(selected_index - 1))
                .unwrap()
                .1
        }
    }

    fn filter_by_province(&self, property_view: &Arc<PropertyView>) -> bool {
        let province = *self.province_filter.lock_ref();
        if province == Province::All {
            true
        } else {
            property_view.property.province == province
        }
    }

    fn filter_by_city(&self, property_view: &Arc<PropertyView>) -> bool {
        let city = *self.city_property_filter.lock_ref();
        if city == ALL_CITIES_STR {
            true
        } else {
            property_view.property.city == city
        }
    }

    fn filter_by_opportunity(&self, property_view: &Arc<PropertyView>) -> bool {
        if *self.opportunity_filter.lock_ref() {
            return true;
        }
        let auction_limit = *self.opportunity_filter_threshold.lock_ref();
        let target_value = property_view.bidinfo.value.to_f64().unwrap_or(0.0) * auction_limit;

        property_view.bidinfo.claim_quantity.to_f64().unwrap_or(0.0) > 1.0
            && target_value > property_view.bidinfo.claim_quantity.to_f64().unwrap_or(0.0)
    }

    fn filter(&self) {
        for property_view in self.property_list.lock_ref().iter() {
            property_view.filtered_in.set_neq(
                self.filter_by_province(property_view)
                    && self.filter_by_city(property_view)
                    && self.filter_by_opportunity(property_view),
            );
        }
    }

    fn update_city_options(&self) {
        let selected_province = *self.province_filter.lock_ref();
        self.city_options.lock_mut().clear();
        self.city_options.lock_mut().push_cloned(ALL_CITIES_STR);
        CITIES_PROVINCES
            .get()
            .unwrap()
            .iter()
            .for_each(|(city, province)| {
                if selected_province == Province::All || selected_province == *province {
                    self.city_options.lock_mut().push_cloned(city);
                }
            });
        let selected_city = *self.city_property_filter.lock_ref();
        if !self.city_options.lock_ref().contains(&selected_city) {
            *self.city_property_filter.lock_mut() = ALL_CITIES_STR;
        }
    }

    fn render_province_options(&self) -> Vec<Dom> {
        let mut provinces = PROVINCES
            .get()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, province)| {
                html!("option", {
                    .attr("value", &(i+1).to_string())
                    .text(province.name())
                })
            })
            .collect::<Vec<Dom>>();

        provinces.insert(
            0,
            html!("option", {
                .attr("value", &0.to_string())
                .text("Todas las provincias")
            }),
        );
        self.update_city_options();
        provinces
    }

    fn render_filter_section(page: Arc<Self>) -> Dom {
        html!("div", {
            .class(&*FILTER_FLEX_CONTAINER_CLASS)
            .children(&mut [
            html!("label", {
                .visible(true)
                .attr("for", "checkbox-opportunities")
                .text("Oportunidades ")
                .child(render_svg_crosshair_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "checkbox")
                .attr("id", "checkbox-opportunities")
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
                .attr("for", "select-province")
                .text("Filtrado por provincia:")
            }),
            html!("select" => HtmlSelectElement, {
                .attr("id", "select-province")
                .attr("alt", "Filtrado por provincia")
                .children(
                    &mut page.render_province_options()[..]
                )
                .with_node!(select => {
                    .event(clone!(page => move |_: events::Change| {
                        let index: usize = select.value().parse().unwrap();
                        *page.province_filter.lock_mut() = PropertyPage::get_province(index);
                        page.update_city_options();
                        page.filter();
                     }))
                })
            }),
            html!("label", {
                .visible(false)
                .attr("for", "select-city")
                .text("Filtrado por ciudad:")
            }),
            html!("select" => HtmlSelectElement, {
                .attr("id", "select-city")
                .attr("alt", "Filtrado por ciudad")
                .children_signal_vec(page.city_options.signal_vec_cloned()
                    .map(clone!(page => move |city|
                        if *page.city_property_filter.lock_ref() == city {
                            html!("option", {
                                .attr("selected", "selected")
                                .attr("value", city)
                                .text(city)
                            })
                        } else {
                            html!("option", {
                                .attr("value", city)
                                .text(city)
                            })
                        })
                    )
                )
                .with_node!(select => {
                    .event(clone!(page => move |_: events::Change| {
                        let lock = page.city_options.lock_ref();
                        let city = lock.iter().find(|c|  **c == select.value()).unwrap();
                        *page.city_property_filter.lock_mut() = city;
                         page.filter();
                     }))
                })
            }),
            html!("div", {
                .attr("id", "mapid")
                .style("z-index", "2147483647")
                .style("height", "250px")
                .style("width", "500px")
                .style("right", "5px")
                .after_inserted(clone!(page => move |_| {
                    page.map.init_map(39.61, -3.69);
                    page.property_list.lock_ref().iter().for_each(|view| {
                        if let Some(coordinates) = view.property.coordinates {
                            page.map.add_marker(&view.property.auction_id,
                                &view.property.auction_id,
                                coordinates.lat(),
                                coordinates.lng())
                        }
                    });

                }))
            }),
            ])
        })
    }

    fn sort_by_province(a: &Arc<PropertyView>, b: &Arc<PropertyView>) -> Ordering {
        a.property.province.cmp(&b.property.province)
    }

    fn sort_by_reverse_province(a: &Arc<PropertyView>, b: &Arc<PropertyView>) -> Ordering {
        b.property.province.cmp(&a.property.province)
    }

    fn sort_by_value(a: &Arc<PropertyView>, b: &Arc<PropertyView>) -> Ordering {
        a.bidinfo.value.cmp(&b.bidinfo.value)
    }

    fn sort_by_reverse_value(a: &Arc<PropertyView>, b: &Arc<PropertyView>) -> Ordering {
        b.bidinfo.value.cmp(&a.bidinfo.value)
    }

    fn sort_by_none(_: &Arc<PropertyView>, _: &Arc<PropertyView>) -> Ordering {
        Ordering::Equal
    }

    fn clear_sortings(&self) {
        *self.province_sorting.lock_mut() = SortingOrder::None;
        *self.value_sorting.lock_mut() = SortingOrder::None;
    }

    fn sorting_by(
        property_sorting: PropertySorting,
    ) -> fn(&Arc<PropertyView>, &Arc<PropertyView>) -> Ordering {
        match property_sorting {
            PropertySorting::ByReverseProvince => PropertyPage::sort_by_reverse_province,
            PropertySorting::ByProvince => PropertyPage::sort_by_province,
            PropertySorting::ByValue => PropertyPage::sort_by_value,
            PropertySorting::ByReverseValue => PropertyPage::sort_by_reverse_value,
            PropertySorting::None => PropertyPage::sort_by_none,
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
                            .class(&*CELL_CLICKABLE_CLASS)
                            .text("Provincia")
                            .child_signal(page.province_sorting.signal().map(|sorting| {
                                match sorting {
                                    SortingOrder::None => Some(Dom::empty()),
                                    SortingOrder::Up => Some(render_svg_arrow_up_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE)),
                                    SortingOrder::Down => Some(render_svg_arrow_down_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE)),
                                }
                            }))
                            .with_node!(_th => {
                                .event(clone!(page => move |_: events::Click| {
                                    let selection = *page.province_sorting.lock_ref();
                                    page.clear_sortings();
                                    match selection {
                                        SortingOrder::None | SortingOrder::Up => {
                                            *page.sorting.lock_mut() = PropertySorting::ByProvince;
                                            *page.province_sorting.lock_mut() = SortingOrder::Down;
                                        },
                                        SortingOrder::Down => {
                                            *page.sorting.lock_mut() = PropertySorting::ByReverseProvince;
                                            *page.province_sorting.lock_mut() = SortingOrder::Up;
                                        },
                                    }
                                }))
                            })
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Ciudad")
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Descripción")
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
                                            *page.sorting.lock_mut() = PropertySorting::ByValue;
                                            *page.value_sorting.lock_mut() = SortingOrder::Down;
                                        },
                                        SortingOrder::Down => {
                                            *page.sorting.lock_mut() = PropertySorting::ByReverseValue;
                                            *page.value_sorting.lock_mut() = SortingOrder::Up;
                                        },
                                    }
                                }))
                            })
                        }),
                    ])
                }),
            ])
        })
    }

    pub fn render(page: Arc<Self>) -> Vec<Dom> {
        vec![
            PropertyPage::render_filter_section(page.clone()),
            html!("table", {
                .class(&*TABLE_CLASS)
                .children(&mut[
                    PropertyPage::render_table_header(page.clone()),
                    html!("tbody", {
                        .class(&*TBODY_CLASS)
                        .children_signal_vec(
                            page.sorting.signal_ref(|filter| *filter)
                            .switch_signal_vec(clone!(page => move |filter| {
                                page.property_list.signal_vec_cloned()
                                .sort_by_cloned(PropertyPage::sorting_by(filter))
                                .map(clone!(page => move |view| {
                                    PropertyView::render(page.clone(), view)
                                }))
                            }))
                        )
                    }),
                ])
            }),
        ]
    }
}
