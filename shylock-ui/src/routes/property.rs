use crate::components::AssetView;
use crate::{ASSETS, CITIES, PROVINCES};

use log;
use shylock_data::provinces::Province;
use shylock_data::types::Asset;
use yew::prelude::*;
use yew_styles::forms::{form_group::FormGroup, form_select::FormSelect};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yewtil::NeqAssign;

const BLANK_OPTION: &str = "";

struct State {
    province: Province,
    city: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct PropertyPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SelectProvince(Province),
    SelectCity(String),
}

impl Component for PropertyPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            state: State {
                province: Province::All,
                city: BLANK_OPTION,
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::SelectProvince(value) => {
                if value == Province::All {
                    self.state.province = Province::All;
                } else {
                    self.state.province = *PROVINCES.get().unwrap().get(&value).unwrap();
                }
                log::debug!("Called update, province: {}", self.state.province.name());
            }
            Msg::SelectCity(value) => {
                if &value == BLANK_OPTION {
                    self.state.city = BLANK_OPTION;
                } else {
                    self.state.city = CITIES.get().unwrap().get(&value[..]).unwrap();
                }
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

impl PropertyPage {
    fn filter_asset_type(&self, asset: &&Asset) -> bool {
        match asset {
            Asset::Property(property) => {
                (self.state.province == Province::All || self.state.province == property.province)
                    && (self.state.city == BLANK_OPTION || self.state.city == property.city)
            }
            Asset::Vehicle(_) => false,
            Asset::Other(_) => false,
        }
    }
}

fn get_asset_province_select(page: &PropertyPage) -> Html {
    html! {
    <FormGroup>
    <FormSelect
        select_size=Size::Medium
        onchange_signal = page.link.callback(|e: ChangeData|
            match e {
            ChangeData::Select(element) => {
               Msg::SelectProvince(element.value().parse::<Province>().unwrap_or(Province::All))
               },
            _ => unreachable!(),
            }
           )
    options=html!{
     <>
     <option selected={page.state.province == Province::All} value={Province::All.name()}>{"Todas las provincias"}</option>
     {
         for PROVINCES.get().unwrap().iter().map(|province| html!{
         <option selected={page.state.province == *province} value={province.name()}>{province.name()}</option>
         })
     }

     </>
    }/>

    </FormGroup>
    }
}

fn get_asset_city_select(page: &PropertyPage) -> Html {
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
        <option selected={page.state.city == BLANK_OPTION} value="">{"Todas las ciudades"}</option>
        {
            for CITIES.get().unwrap().iter().map(|city| html!{
                <option selected={&page.state.city == city} value={city}>{city}</option>
            })
        }
        </>
    }/>

    </FormGroup>
    }
}

fn get_items(asset: Html) -> Html {
    html! {
        <Item
            layouts=vec!(ItemLayout::ItXl(4))
        >
            {asset}
        </Item>
    }
}
