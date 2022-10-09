use std::sync::Arc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use shylock_data::provinces::Province;
use web_sys::HtmlSelectElement;

use crate::feather::render_svg_crosshair_icon;
use crate::global::{
    CELL_CLASS, DEFAULT_ICON_COLOR, FILTER_FLEX_CONTAINER_CLASS, ROW_CLASS, TABLE_CLASS,
    TBODY_CLASS, THEAD_CLASS,
};
use crate::{
    global::{CITIES_PROVINCES, PROVINCES},
    property_view::PropertyView,
};
use crate::{MyMap, THUNDERFOREST_API_KEY};

const ALL_CITIES_STR: &str = "Todas las ciudades";

pub struct PropertyPage {
    property_list: MutableVec<Arc<PropertyView>>,
    city_options: MutableVec<&'static str>,
    city_property_filter: Mutable<&'static str>,
    province_property_filter: Mutable<Province>,
    pub map: MyMap,
}

impl PropertyPage {
    pub fn new(property_list: MutableVec<Arc<PropertyView>>) -> Arc<Self> {
        Arc::new(Self {
            property_list,
            city_options: MutableVec::new(),
            city_property_filter: Mutable::new(ALL_CITIES_STR),
            province_property_filter: Mutable::new(Province::All),
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
        let province = *self.province_property_filter.lock_ref();
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

    fn filter(&self) {
        for property_view in self.property_list.lock_ref().iter() {
            property_view.filtered_in.set_neq(
                self.filter_by_province(property_view) && self.filter_by_city(property_view),
            );
        }
    }

    fn update_city_options(&self) {
        let selected_province = *self.province_property_filter.lock_ref();
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
                        *page.province_property_filter.lock_mut() = PropertyPage::get_province(index);
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

    fn render_table_header(&self) -> Dom {
        html!("thead", {
            .class(&*THEAD_CLASS)
            .children(&mut[
                html!("tr", {
                    .class(&*ROW_CLASS)
                    .children(&mut [
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .child(render_svg_crosshair_icon(DEFAULT_ICON_COLOR))
                        }),
                        html!("th", {
                            .class(&*CELL_CLASS)
                            .text("Provincia")
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
            PropertyPage::render_filter_section(page.clone()),
            html!("table", {
                .class(&*TABLE_CLASS)
                .children(&mut[
                    page.render_table_header(),
                    html!("tbody", {
                        .class(&*TBODY_CLASS)
                        .children_signal_vec(page.property_list.signal_vec_cloned()
                            .map(clone!(page => move |view| {
                                PropertyView::render(page.clone(), view)
                            }))
                        )
                    }),
                ])
            }),
        ]
    }
}
