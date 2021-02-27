use crate::global::AUCTIONS;

use num_format::{Buffer, Locale};
use rust_decimal::prelude::*;
use shylock_data::{types::Asset, BidInfo};
use std::{cmp::min, ops::Add};
use substring::Substring;
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
                  header=Some(html!{<div>{"Inmueble"}</div>})
                  body=Some(html!{<div>{summarize(&property.description)}</div>})
                  footer=Some(html!{<div><b>{get_valuation(&property.bidinfo, &property.auction_id)}{" €"}</b></div>}) />
            },
            Asset::Vehicle(vehicle) => html! {
                <Card
                  key=self.props.position.to_string()
                  card_size=Size::Medium
                  card_palette=Palette::Clean
                  card_style=Style::Outline
                  header=Some(html!{<div>{"Vehículo"}</div>})
                  body=Some(html!{<div>{summarize(&vehicle.description)}</div>})
                  footer=Some(html!{<div><b>{get_valuation(&vehicle.bidinfo, &vehicle.auction_id)}{" €"}</b></div>}) />
            },
            Asset::Other(other) => html! {
                <Card
                  key=self.props.position.to_string()
                  card_size=Size::Medium
                  card_palette=Palette::Clean
                  card_style=Style::Outline
                  header=Some(html!{<div>{"Bien"}</div>})
                  body=Some(html!{<div>{summarize(&other.description)}</div>})
                  footer=Some(html!{<div><b>{get_valuation(&other.bidinfo, &other.auction_id)}{" €"}</b></div>}) />
            },
        }
    }
}

fn get_valuation(bidinfo: &Option<BidInfo>, auction_id: &str) -> String {
    let mut buf = Buffer::default();

    let valuation = if bidinfo.is_some() {
        bidinfo.as_ref().unwrap().value
    } else {
        AUCTIONS
            .get()
            .unwrap()
            .get(auction_id)
            .unwrap()
            .bidinfo
            .value
    };

    // Write "1,000,000" into the buffer...
    buf.write_formatted(&valuation.trunc().to_u64().unwrap_or(0), &Locale::es);

    // Get a view into the buffer as a &str...
    format!(
        "{},{}",
        buf.as_str(),
        valuation.fract().to_u32().unwrap_or(0)
    )
}

const TEXT_LIMIT: usize = 300;

fn summarize(text: &str) -> String {
    let str_min = min(text.chars().count(), TEXT_LIMIT);
    let mut new_text = text.substring(0, str_min).to_string();

    if new_text.chars().count() == TEXT_LIMIT {
        new_text = new_text.add("...");
    }

    new_text
}
