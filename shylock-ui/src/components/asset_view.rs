use crate::route::AppRoute;
use crate::utils::{format_valuation, get_bidinfo, is_targeted_asset, summarize};
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};

use shylock_data::{types::Asset, Other, Property, Vehicle};
use yew::prelude::*;
use yew_assets::business_assets::{BusinessAssets, BusinessIcon};
use yew_styles::card::Card;
use yew_styles::styles::{Palette, Size, Style};
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub position: usize,
    pub asset: &'static Asset,
}

pub struct AssetView {
    props: Props,
    router: RouteAgentDispatcher,
    link: ComponentLink<Self>,
}

pub enum Msg {
    PropertyClicked(MouseEvent),
    VehicleClicked(MouseEvent),
    OtherClicked(MouseEvent),
}
impl Component for AssetView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            router: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PropertyClicked(_) => {
                self.router.send(RouteRequest::ChangeRoute(
                    AppRoute::PropertyDetail(self.props.position).into_route(),
                ));
            }
            Msg::VehicleClicked(_) => {
                self.router.send(RouteRequest::ChangeRoute(
                    AppRoute::VehicleDetail(self.props.position).into_route(),
                ));
            }
            Msg::OtherClicked(_) => {
                self.router.send(RouteRequest::ChangeRoute(
                    AppRoute::OtherDetail(self.props.position).into_route(),
                ));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match self.props.asset {
            Asset::Property(property) => {
                html! {
                    <Card
                      key=self.props.position.to_string()
                      card_size=Size::Medium
                      card_palette=Palette::Clean
                      card_style=Style::Outline
                      class_name="pointer"
                      onclick_signal=self.link.callback(Msg::PropertyClicked)
                      header=Some(self.get_property_header(property))
                      body=Some(html!{<div>{summarize(&property.description)}</div>})
                      footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&property.bidinfo, &property.auction_id).value)}{" €"}</b></div>}) />
                }
            }
            Asset::Vehicle(vehicle) => {
                html! {
                  <Card
                    key=self.props.position.to_string()
                    card_size=Size::Medium
                    card_palette=Palette::Clean
                    card_style=Style::Outline
                    class_name="pointer"
                    onclick_signal=self.link.callback(Msg::VehicleClicked)
                    header=Some(self.get_vehicle_header(vehicle))
                    body=Some(html!{<div>{summarize(&vehicle.description)}</div>})
                    footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&vehicle.bidinfo, &vehicle.auction_id).value)}{" €"}</b></div>}) />
                }
            }
            Asset::Other(other) => {
                html! {
                  <Card
                    key=self.props.position.to_string()
                    card_size=Size::Medium
                    card_palette=Palette::Clean
                    card_style=Style::Outline
                    class_name="pointer"
                    onclick_signal=self.link.callback(Msg::OtherClicked)
                    header=Some(self.get_other_header(other))
                    body=Some(html!{<div>{summarize(&other.description)}</div>})
                    footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&other.bidinfo, &other.auction_id).value)}{" €"}</b></div>}) />
                }
            }
        }
    }
}

impl AssetView {
    fn get_property_header(&self, property: &Property) -> Html {
        if is_targeted_asset(&property.bidinfo, &property.auction_id) {
            html! {
                <>
                  <BusinessAssets
                    icon = BusinessIcon::Target
                    fill = "#fff"
                    size = ("30".to_string(),"30".to_string()) />
                  {" "}{&property.city}{" "}{&property.province.name()}{" "}
                </>
            }
        } else {
            html! {
                <>
                {&property.city}{" "}{&property.province.name()}{" "}
                </>
            }
        }
    }

    fn get_vehicle_header(&self, vehicle: &Vehicle) -> Html {
        if is_targeted_asset(&vehicle.bidinfo, &vehicle.auction_id) {
            html! {
                <>
                  <BusinessAssets
                    icon = BusinessIcon::Target
                    fill = "#fff"
                    size = ("30".to_string(),"30".to_string()) />
                  {" "}{&vehicle.model}{" "}{&vehicle.brand}{" "}
                </>
            }
        } else {
            html! {
                <>
                {&vehicle.model}{" "}{&vehicle.brand}{" "}
                </>
            }
        }
    }

    fn get_other_header(&self, other: &Other) -> Html {
        if is_targeted_asset(&other.bidinfo, &other.auction_id) {
            html! {
                <>
                  <BusinessAssets
                    icon = BusinessIcon::Target
                    fill = "#fff"
                    size = ("30".to_string(),"30".to_string()) />
                  {" "}{&other.category.name()}{" "}
                </>
            }
        } else {
            html! {
                <>
                {&other.category.name()}{" "}
                </>
            }
        }
    }
}
