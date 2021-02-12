use crate::Asset;

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub position: usize,
    pub asset: &'static Asset,
}

pub struct AssetView {
    props: Props,
    // link: ComponentLink<Self>,
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
                <div key=self.props.position.to_string() class="asset_container">
                <div class="asset_name">{"Bien inmueble"}</div>
                <div class="asset_auction">{&property.auction_id}</div>
                </div>
            },
            Asset::Vehicle(vehicle) => html! {
                <div key=self.props.position.to_string() class="asset_container">
                <div class="asset_name">{"Vehículo"}</div>
                <div class="asset_auction">{&vehicle.auction_id}</div>
                </div>
            },
            Asset::Other(other) => html! {
                <div key=self.props.position.to_string() class="asset_container">
                <div class="asset_name">{"Bien mueble"}</div>
                <div class="asset_auction">{&other.auction_id}</div>
                </div>
            },
        }
    }
}
