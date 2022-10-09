use crate::feather::render_svg_crosshair_icon;
use crate::global::{
    DEFAULT_ICON_COLOR, NAVBAR_CLASS, NAVITEM_CLASS, NAV_LINK_CLASS, NAV_SELECTED_CLASS,
    NAV_UL_CLASS, ROOT_CLASS, SECTION_CLASS,
};
use crate::other_asset_page::OtherAssetPage;
use crate::other_asset_view::OtherAssetView;
use crate::property_page::PropertyPage;
use crate::property_view::PropertyView;
use crate::route::Route;
use crate::vehicle_page::VehiclePage;
use crate::vehicle_view::VehicleView;
use build_time::build_time_local;
use dominator::{clone, html, link, routing, stylesheet, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;
use std::sync::Arc;

pub struct App {
    property_page: Arc<PropertyPage>,
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
            property_page: PropertyPage::new(property_list),
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
                        Self::render_button(&app, "Vehículos", Route::Vehicles),
                        Self::render_button(&app, "Otros bienes", Route::OtherAssets),
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
                        render_svg_crosshair_icon(DEFAULT_ICON_COLOR),
                    ])
                    .text(" para bienes que merezcan la pena. ")
                }),
                html!("p", {
                    .children(&mut[
                        link!("mailto:contacto@coditia.com", {
                            .attr("alt", "email contacto")
                            .text("Escríbeme")
                        }),
                     ])
                     .text(" para cualquier duda o sugerencia.")
                })
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

    fn render_footer() -> Dom {
        html!("footer", {
        .children(&mut[
            html!("p",{
                .text(build_time_local!("Última actualización: %e de %B del %Y"))
            })]
        )})
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
                Self::render_vehicles(app.clone()),
                Self::render_other_assets(app),
                Self::render_footer(),
            ])
        })
    }
}
