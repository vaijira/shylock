use crate::{route::Route, ASSETS, AUCTIONS};
use crate::{routes::Home, CITIES, MAX_AUCTION_VALUE, PROVINCES};

use shylock_data::types::{Asset, Auction};
use std::collections::{BTreeSet, HashMap};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{Palette, Size};
use yew_styles::text::{Text, TextType};
use yewtil::future::LinkFuture;

struct State {
    get_assets_loaded: bool,
    get_auctions_loaded: bool,
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
                    let response = reqwasm::Request::get("/tmp/assets.min.json")
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
                match ASSETS.set(assets) {
                    Err(_) => log::error!("Not able to set global assets"),
                    _ => (),
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
                    let response = reqwasm::Request::get("/tmp/auctions.min.json")
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
                match AUCTIONS.set(auctions) {
                    Err(_) => log::error!("Not able to set global auctions"),
                    _ => (),
                }
                log::info!("Loaded {} auctions", AUCTIONS.get().unwrap().len());
                self.state.get_auctions_loaded = true;
                set_global_info();
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.state.get_assets_loaded && self.state.get_auctions_loaded {
            let render = Router::render(|switch: Route| match switch {
                Route::HomePage => html! {<Home/>},
            });

            html! {
                <Router<Route, ()> render=render/>
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
                <div class="center">
                    <Spinner
                        spinner_type=SpinnerType::Circle
                        spinner_size=Size::Big
                        spinner_palette=Palette::Info/>
                    <Text
                        text_type=TextType::Plain
                        text_size=Size::Medium
                        plain_text={text}
                        html_text=None/>
                </div>
            }
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn set_global_info() {
    match MAX_AUCTION_VALUE.set(
        AUCTIONS
            .get()
            .unwrap()
            .iter()
            .map(|(_, auction)| auction.value)
            .max()
            .unwrap(),
    ) {
        Err(_) => log::error!("Not able to set max auction value"),
        _ => (),
    };

    match PROVINCES.set(
        ASSETS
            .get()
            .unwrap()
            .iter()
            .filter_map(|asset| match asset {
                Asset::Property(property) => Some(property.province.name()),
                Asset::Vehicle(_) => None,
                Asset::Other(_) => None,
            })
            .collect::<BTreeSet<&str>>(),
    ) {
        Err(_) => log::error!("Not able to set provinces"),
        _ => (),
    };

    match CITIES.set(
        ASSETS
            .get()
            .unwrap()
            .iter()
            .filter_map(|asset| match asset {
                Asset::Property(property) => Some(&property.city[..]),
                Asset::Vehicle(_) => None,
                Asset::Other(_) => None,
            })
            .collect::<BTreeSet<&str>>(),
    ) {
        Err(_) => log::error!("Not able to set cities"),
        _ => (),
    };
}
