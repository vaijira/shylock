use crate::global::{format_valuation, get_bidinfo, summarize};
use crate::route::{AppAnchor, AppRoute};

use shylock_data::types::Asset;
use yew::prelude::*;
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
}

impl Component for AssetView {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match self.props.asset {
            Asset::Property(property) => html! {
                <Card
                  key=self.props.position.to_string()
                  card_size=Size::Medium
                  card_palette=Palette::Clean
                  card_style=Style::Outline
                  header=Some(html!{<AppAnchor route=AppRoute::PropertyDetail(self.props.position)>{"Inmueble"}</AppAnchor>})
                  body=Some(html!{<div>{summarize(&property.description)}</div>})
                  footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&property.bidinfo, &property.auction_id).value)}{" €"}</b></div>}) />
            },
            Asset::Vehicle(vehicle) => html! {
                <Card
                  key=self.props.position.to_string()
                  card_size=Size::Medium
                  card_palette=Palette::Clean
                  card_style=Style::Outline
                  header=Some(html!{<div>{"Vehículo"}</div>})
                  body=Some(html!{<div>{summarize(&vehicle.description)}</div>})
                  footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&vehicle.bidinfo, &vehicle.auction_id).value)}{" €"}</b></div>}) />
            },
            Asset::Other(other) => html! {
                <Card
                  key=self.props.position.to_string()
                  card_size=Size::Medium
                  card_palette=Palette::Clean
                  card_style=Style::Outline
                  header=Some(html!{<div>{"Bien"}</div>})
                  body=Some(html!{<div>{summarize(&other.description)}</div>})
                  footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&other.bidinfo, &other.auction_id).value)}{" €"}</b></div>}) />
            },
        }
    }
}
