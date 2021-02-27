use crate::routes::{HomePage, OtherPage, PropertyPage, VehiclePage};
use crate::{
    global::{set_global_info, ASSETS, AUCTIONS},
    route::{AppRouter, FragmentOnlyRoute},
};
use shylock_data::types::{Asset, Auction};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{AlignSelf, Item, ItemLayout},
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::text::{Text, TextType};
use yew_styles::{
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Size, Style},
};
use yewtil::future::LinkFuture;

const HOME_MENU: &str = "inicio";
const PROPERTY_MENU: &str = "inmueble";
const VEHICLE_MENU: &str = "vehiculo";
const OTHER_MENU: &str = "otro";

struct State {
    get_assets_loaded: bool,
    get_auctions_loaded: bool,
    selected_menu: &'static str,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    GetAssets,
    GotAssets(Vec<Asset>),
    GetAuctions,
    GotAuctions(HashMap<String, Auction>),
    ChangeMenu(&'static str),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetAssets);
        Self {
            state: State {
                get_assets_loaded: false,
                get_auctions_loaded: false,
                selected_menu: HOME_MENU,
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetAssets => {
                log::debug!("Get assets");
                self.link.send_future(async {
                    log::debug!("Request assets");
                    let response = reqwasm::Request::get("tmp/assets.min.json")
                        .send()
                        .await
                        .expect("Unable to request assets");
                    log::debug!("Request assets sent");
                    let assets: Vec<Asset> = match response.json().await {
                        Ok(result) => result,
                        Err(text) => {
                            log::error!("Response error getting assets: {}", text);
                            Vec::new()
                        }
                    };
                    log::debug!("Response received");
                    Msg::GotAssets(assets)
                });

                true
            }
            Msg::GotAssets(assets) => {
                if ASSETS.set(assets).is_err() {
                    log::error!("Not able to set global assets")
                }
                log::info!("Loaded {} assets", ASSETS.get().unwrap().len());
                self.state.get_assets_loaded = true;
                self.link.send_message(Msg::GetAuctions);
                true
            }
            Msg::GetAuctions => {
                log::info!("Get auctions");
                self.link.send_future(async {
                    log::debug!("Request auctions");
                    let response = reqwasm::Request::get("tmp/auctions.min.json")
                        .send()
                        .await
                        .expect("Unable to request auctions");
                    log::debug!("Request auctions sent");
                    let auctions: HashMap<String, Auction> = match response.json().await {
                        Ok(result) => result,
                        Err(text) => {
                            log::error!("Response error getting auctions: {}", text);
                            HashMap::new()
                        }
                    };
                    log::debug!("Response received");
                    Msg::GotAuctions(auctions)
                });

                true
            }
            Msg::GotAuctions(auctions) => {
                if AUCTIONS.set(auctions).is_err() {
                    log::error!("Not able to set global auctions");
                }
                log::info!("Loaded {} auctions", AUCTIONS.get().unwrap().len());
                self.state.get_auctions_loaded = true;
                set_global_info();
                true
            }
            Msg::ChangeMenu(selection) => {
                log::debug!("Change Menu selection: {}", selection);
                if self.state.selected_menu == selection {
                    false
                } else {
                    self.state.selected_menu = selection;
                    true
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.state.get_assets_loaded && self.state.get_auctions_loaded {
            let render =
                Router::render(|switch: FragmentOnlyRoute<AppRouter>| match switch.inner {
                    AppRouter::Properties => html! { <PropertyPage/> },
                    AppRouter::Vehicles => html! { <VehiclePage/> },
                    AppRouter::Others => html! { <OtherPage/> },
                    AppRouter::Home => html! { <HomePage/> },
                    AppRouter::Root => html! { <HomePage/> },
                });
            let redirect =
                Router::redirect(|_: Route<()>| FragmentOnlyRoute::from(AppRouter::Root));
            html! {
                <>
                { self.get_navbar() }
                <Router<FragmentOnlyRoute<AppRouter>, ()> render=render redirect=redirect/>
                </>
            }
        } else {
            let text = if self.state.get_assets_loaded {
                "Cargando subastas ..."
            } else if self.state.get_auctions_loaded {
                "Terminado"
            } else {
                "Cargando bienes ..."
            };
            html! {
              <Container
                direction=Direction::Column wrap=Wrap::Nowrap
                justify_content=JustifyContent::Center(Mode::NoMode)
                align_items=AlignItems::Center(Mode::NoMode)>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                  <Spinner
                    spinner_type=SpinnerType::Circle
                    spinner_size=Size::Big
                    spinner_palette=Palette::Info/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Big
                    plain_text={text}
                    html_text=None/>
                </Item>
              </Container>
            }
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

impl App {
    fn get_navbar(&self) -> Html {
        html! {
          <Navbar class_name="navbar-router" fixed=Fixed::None navbar_style=Style::Outline navbar_palette=Palette::Clean>
            <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
              <NavbarItem
                  class_name="navbar-route"
                  active={self.state.selected_menu == HOME_MENU}
                  onclick_signal=self.link.callback(move |_| Msg::ChangeMenu(HOME_MENU))>
                  <RouterAnchor<FragmentOnlyRoute<AppRouter>>route=FragmentOnlyRoute::from(AppRouter::Home)>{"Inicio"}</RouterAnchor<FragmentOnlyRoute<AppRouter>>>
              </NavbarItem>
              <NavbarItem
                  class_name="navbar-route"
                  active={self.state.selected_menu == PROPERTY_MENU}
                  onclick_signal=self.link.callback(move |_| Msg::ChangeMenu(PROPERTY_MENU))>
                  <RouterAnchor<FragmentOnlyRoute<AppRouter>>route=FragmentOnlyRoute::from(AppRouter::Properties)>{"Inmuebles"}</RouterAnchor<FragmentOnlyRoute<AppRouter>>>
              </NavbarItem>
              <NavbarItem
                  class_name="navbar-route"
                  active={self.state.selected_menu == VEHICLE_MENU}
                  onclick_signal=self.link.callback(move |_| Msg::ChangeMenu(VEHICLE_MENU))>
                  <RouterAnchor<FragmentOnlyRoute<AppRouter>>route=FragmentOnlyRoute::from(AppRouter::Vehicles)>{"Vehículos"}</RouterAnchor<FragmentOnlyRoute<AppRouter>>>
              </NavbarItem>
              <NavbarItem
                  class_name="navbar-route"
                  active={self.state.selected_menu == OTHER_MENU}
                  onclick_signal=self.link.callback(move |_| Msg::ChangeMenu(OTHER_MENU))>
                  <RouterAnchor<FragmentOnlyRoute<AppRouter>>route=FragmentOnlyRoute::from(AppRouter::Others)>{"Otros bienes"}</RouterAnchor<FragmentOnlyRoute<AppRouter>>>
              </NavbarItem>
            </NavbarContainer>
          </Navbar>
        }
    }
}
