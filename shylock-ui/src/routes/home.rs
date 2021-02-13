use crate::components::AssetView;
use crate::{ASSETS, CITIES, PROVINCES};

use log;
use shylock_data::types::Asset;
use yew::prelude::*;
use yew_styles::forms::{form_group::FormGroup, form_select::FormSelect};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yewtil::NeqAssign;

const PROPERTY_OPTION: &str = "inmueble";
const VEHICLE_OPTION: &str = "vehiculo";
const OTHER_OPTION: &str = "otro";
const BLANK_OPTION: &str = "";

struct State {
    asset_type: &'static str,
    province: &'static str,
    city: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Home {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SelectAssetType(String),
    SelectProvince(String),
    SelectCity(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            state: State {
                asset_type: BLANK_OPTION,
                province: BLANK_OPTION,
                city: BLANK_OPTION,
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::SelectAssetType(value) => {
                match &value[..] {
                    PROPERTY_OPTION => self.state.asset_type = PROPERTY_OPTION,
                    VEHICLE_OPTION => self.state.asset_type = VEHICLE_OPTION,
                    OTHER_OPTION => self.state.asset_type = OTHER_OPTION,
                    _ => self.state.asset_type = BLANK_OPTION,
                }
                log::debug!("Called update, asset type: {}", self.state.asset_type);
            }
            Msg::SelectProvince(value) => {
                self.state.province = PROVINCES.get().unwrap().get(&value[..]).unwrap();
                log::debug!("Called update, province: {}", self.state.province);
            }
            Msg::SelectCity(value) => {
                self.state.city = CITIES.get().unwrap().get(&value[..]).unwrap();
                log::debug!("Called update, city: {}", self.state.city);
            }
        }

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
          <>
            <Container direction=Direction::Row wrap=Wrap::Wrap>
            <Item layouts=vec!(ItemLayout::ItXs(2))>
            {get_asset_type_select(self)}
            </Item>
            <Item layouts=vec!(ItemLayout::ItXs(2))>
            {get_asset_province_select(self)}
            </Item>
            <Item layouts=vec!(ItemLayout::ItXs(2))>
            {get_asset_city_select(self)}
            </Item>

            </Container>
            <Container direction=Direction::Row wrap=Wrap::Wrap>
            {assets.drain(..).map(|x| get_items(x)).collect::<Html>()}
            </Container>
         </>
        }
    }
}

impl Home {
    fn filter_asset_type(&self, asset: &&Asset) -> bool {
        if self.state.asset_type == BLANK_OPTION {
            true
        } else {
            match asset {
                Asset::Property(_) => self.state.asset_type == PROPERTY_OPTION,
                Asset::Vehicle(_) => self.state.asset_type == VEHICLE_OPTION,
                Asset::Other(_) => self.state.asset_type == OTHER_OPTION,
            }
        }
    }
}

fn get_asset_type_select(page: &Home) -> Html {
    html! {
    <FormGroup>
    <FormSelect
        select_size=Size::Medium
        onchange_signal = page.link.callback(|e: ChangeData|
            match e {
            ChangeData::Select(element) => {
               Msg::SelectAssetType(element.value())
               },
            _ => unreachable!(),
            }
           )
    options=html!{
     <>
     <option value="">{"Todo tipo de bienes"}</option>
     <option value={PROPERTY_OPTION}>{"Inmuebles"}</option>
     <option value={VEHICLE_OPTION}>{"Vehículos"}</option>
     <option value={OTHER_OPTION}>{"Otros"}</option>
     </>
    }/>

    </FormGroup>
    }
}

fn get_asset_province_select(page: &Home) -> Html {
    html! {
    <FormGroup>
    <FormSelect
        select_size=Size::Medium
        onchange_signal = page.link.callback(|e: ChangeData|
            match e {
            ChangeData::Select(element) => {
               Msg::SelectProvince(element.value())
               },
            _ => unreachable!(),
            }
           )
    options=html!{
     <>
     <option value="">{"Todas las provincias"}</option>
     { for PROVINCES.get().unwrap().iter().map(|province| html!{<option value={province}>{province}</option>}) }

     </>
    }/>

    </FormGroup>
    }
}

fn get_asset_city_select(page: &Home) -> Html {
    html! {
    <FormGroup>
    <FormSelect
        select_size=Size::Medium
        onchange_signal = page.link.callback(|e: ChangeData|
            match e {
            ChangeData::Select(element) => {
               Msg::SelectCity(element.value())
               },
            _ => unreachable!(),
            }
           )
    options=html!{
        <>
        <option value="">{"Todas las ciudades"}</option>
        { for CITIES.get().unwrap().iter().map(|city| html!{<option value={city}>{city}</option>}) }
        </>
    }/>

    </FormGroup>
    }
}

fn get_items(asset: Html) -> Html {
    html! {
        <Item
            layouts=vec!(ItemLayout::ItXl(3))
        >
            {asset}
        </Item>
    }
}
