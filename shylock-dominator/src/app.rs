use crate::feather::render_svg_crosshair_icon;
use crate::global::{
    DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE, NAVBAR_CLASS, NAVITEM_CLASS, NAV_LINK_CLASS,
    NAV_SELECTED_CLASS, NAV_UL_CLASS, ROOT_CLASS, SECTION_CLASS,
};
use crate::other_asset_page::OtherAssetPage;
use crate::other_asset_view::OtherAssetView;
use crate::property_map_page::PropertyMapPage;
use crate::property_page::PropertyPage;
use crate::property_view::PropertyView;
use crate::route::Route;
use crate::vehicle_page::VehiclePage;
use crate::vehicle_view::VehicleView;
use dominator::{clone, html, link, routing, stylesheet, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;
use std::sync::Arc;

pub struct App {
    property_page: Arc<PropertyPage>,
    property_map_page: Arc<PropertyMapPage>,
    vehicle_page: Arc<VehiclePage>,
    other_assets_page: Arc<OtherAssetPage>,
    route: Mutable<Route>,
}

impl App {
    pub fn new(
        property_list: MutableVec<Arc<PropertyView>>,
        vehicles: MutableVec<Arc<VehicleView>>,
        other_assets: MutableVec<Arc<OtherAssetView>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            property_page: PropertyPage::new(property_list.clone()),
            property_map_page: PropertyMapPage::new(property_list),
            vehicle_page: VehiclePage::new(vehicles),
            other_assets_page: OtherAssetPage::new(other_assets),
            route: Mutable::new(Route::default()),
        })
    }

    pub fn route(&self) -> impl Signal<Item = Route> {
        self.route.signal()
    }

    fn render_button(app: &App, text: &str, route: Route) -> Dom {
        html!("li", {
            .class(&*NAVITEM_CLASS)
            .children(&mut [
                link!(route.to_url(), {
                    .attr("alt", text)
                    .text(text)
                    .class(&*NAV_LINK_CLASS)
                    .class_signal(&*NAV_SELECTED_CLASS, app.route().map(move |x| x == route))
                })
            ])
        })
    }

    fn render_header(app: Arc<Self>) -> Dom {
        html!("header", {
        .children(&mut[
            html!("nav",{
            .class(&*NAVBAR_CLASS)
            .children(&mut [
                html!("ul", {
                .class(&*NAV_UL_CLASS)
                .children(&mut [
                        Self::render_button(&app, "Inicio", Route::Home),
                        Self::render_button(&app, "Inmuebles", Route::Properties),
                        Self::render_button(&app, "Mapa Inmuebles", Route::PropertiesMap),
                        Self::render_button(&app, "Vehículos", Route::Vehicles),
                        Self::render_button(&app, "Otros bienes", Route::OtherAssets),
                        Self::render_button(&app, "Estadísticas", Route::Statistics),
                    ])
                }),
            ])
        })]
        )})
    }

    fn render_home(app: Arc<Self>) -> Dom {
        html!("section", {
            .visible_signal(app.route().map(move |x| x == Route::Home))
            .children(&mut[
                html!("p", {
                    .text("Coditia te ayuda a buscar las mejores subastas. ")
                    .text("Busca ")
                    .children(&mut[
                        render_svg_crosshair_icon(DEFAULT_ICON_COLOR, DEFAULT_ICON_SIZE),
                    ])
                    .text(" para bienes que merezcan la pena. ")
                }),
                html!("p", {
                    .text("Enlaces de interés:")
                    .children(&mut[
                        html!("p", {
                            .child(html!("a", {
                                .attr("alt", "Herramienta de valoración de inmuebles BBVA valora")
                                .attr("href", "https://www.bbva.es/personas/experiencias/bbva-valora/analiza-vivienda.html#")
                                .attr("target", "_blank")
                                .attr("rel", "external nofollow")
                                .text("BBVA Valora")
                            }))
                            .text(" herramienta del BBVA para valorar inmuebles.")
                        }),
                        html!("p", {
                            .child(html!("a", {
                                .attr("alt", "Herramienta de valoración de inmuebles de idealista")
                                .attr("href", "https://www.idealista.com/valoracion-de-inmuebles/")
                                .attr("target", "_blank")
                                .attr("rel", "external nofollow")
                                .text("Herramienta de idealista")
                            }))
                            .text(" para valoración de inmuebles.")
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_properties(app: Arc<Self>) -> Dom {
        html!("section", {
            .class(&*SECTION_CLASS)
            .visible_signal(app.route().map(move |x| x == Route::Properties))
            .children(PropertyPage::render(app.property_page.clone()))
        })
    }

    fn render_properties_map(app: Arc<Self>) -> Dom {
        html!("section", {
            .class(&*SECTION_CLASS)
            .visible_signal(app.route().map(move |x| x == Route::PropertiesMap))
            .children(PropertyMapPage::render(app.property_map_page.clone()))
        })
    }

    fn render_vehicles(app: Arc<Self>) -> Dom {
        html!("section", {
            .class(&*SECTION_CLASS)
            .visible_signal(app.route().map(move |x| x == Route::Vehicles))
            .children(VehiclePage::render(app.vehicle_page.clone()))
        })
    }

    fn render_other_assets(app: Arc<Self>) -> Dom {
        html!("section", {
            .class(&*SECTION_CLASS)
            .visible_signal(app.route().map(move |x| x == Route::OtherAssets))
            .children(OtherAssetPage::render(app.other_assets_page.clone()))
        })
    }

    fn render_statistics(app: Arc<Self>) -> Dom {
        html!("section", {
            .class(&*SECTION_CLASS)
            .visible_signal(app.route().map(move |x| x == Route::Statistics))
            .children(&mut [
                html!("img", {
                    .attr("alt", "Subastas abiertas al mes")
                    .attr("aria-label", "Subastas abiertas por mes")
                    .attr("src", "images/auctions_by_month.svg")
                }),
            ])
        })
    }

    pub fn render(app: Arc<Self>) -> Dom {
        stylesheet!("html", {
            .style("font-family", "arial")
        });

        html!("section", {
            .class(&*ROOT_CLASS)

            // Update the Route when the URL changes
            .future(routing::url()
                .signal_ref(|url| Route::from_url(url))
                .for_each(clone!(app => move |route| {
                    app.route.set_neq(route);
                    async {}
                })))

            .children(&mut [
                Self::render_header(app.clone()),
                Self::render_home(app.clone()),
                Self::render_properties(app.clone()),
                Self::render_properties_map(app.clone()),
                Self::render_vehicles(app.clone()),
                Self::render_other_assets(app.clone()),
                Self::render_statistics(app),
                crate::footer::render_footer(),
            ])
        })
    }
}
