use crate::components::AssetView;
use crate::global::ASSETS;

use shylock_data::types::Asset;
use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct OtherPage {
    props: Props,
}

pub enum Msg {}

impl Component for OtherPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!("Called change");
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut assets: Vec<Html> = ASSETS
            .get()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, asset)| self.filter_asset_type(asset))
            .map(|(i, asset)| {
                html! {
                    <AssetView position=i asset=asset />
                }
            })
            .collect();

        html! {
           <Container direction=Direction::Row wrap=Wrap::Wrap>
           {assets.drain(..).map(get_items).collect::<Html>()}
           </Container>
        }
    }
}

impl OtherPage {
    fn filter_asset_type(&self, asset: &&Asset) -> bool {
        match asset {
            Asset::Property(_) => false,
            Asset::Vehicle(_) => false,
            Asset::Other(_) => true,
        }
    }
}

fn get_items(asset: Html) -> Html {
    html! {
        <Item
            layouts=vec!(ItemLayout::ItXl(4))>
            {asset}
        </Item>
    }
}
