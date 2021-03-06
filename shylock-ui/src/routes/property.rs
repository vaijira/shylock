use crate::components::pagination::ITEMS_PER_PAGE;
use crate::components::{AssetView, Pagination};
use crate::global::{ASSETS, CITIES, PROVINCES};

use shylock_data::provinces::Province;
use shylock_data::types::Asset;
use std::cmp::min;
use yew::prelude::*;
use yew_styles::forms::form_select::FormSelect;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yewtil::NeqAssign;

const BLANK_OPTION: &str = "";

struct State {
    current_page: usize,
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
    PaginationChanged(usize),
}

impl Component for PropertyPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            state: State {
                current_page: 0,
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
                self.state.current_page = 0;
                log::debug!("Called update, province: {}", self.state.province.name());
            }
            Msg::SelectCity(value) => {
                if value == BLANK_OPTION {
                    self.state.city = BLANK_OPTION;
                } else {
                    self.state.city = CITIES.get().unwrap().get(&value[..]).unwrap();
                }
                self.state.current_page = 0;
                log::debug!("Called update, city: {}", self.state.city);
            }
            Msg::PaginationChanged(page) => {
                self.state.current_page = page;
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!("Called change");
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let items_count = ASSETS
            .get()
            .unwrap()
            .iter()
            .filter(|asset| self.filter_asset_type(asset))
            .count();
        let first_item = self.state.current_page * ITEMS_PER_PAGE;
        let last_item = min(first_item + ITEMS_PER_PAGE, items_count);

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

        let pagination_callback = self.link.callback(Msg::PaginationChanged);
        log::debug!(
            "items_count: {}, current_page: {}, first_item: {}, last_item: {}",
            items_count,
            self.state.current_page,
            first_item,
            last_item
        );
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
            <Item layouts=vec!(ItemLayout::ItXs(12))>
            <Pagination
              items_count=items_count
              current_page=self.state.current_page
              callback=pagination_callback />
            </Item>
            </Container>

            <Container direction=Direction::Row wrap=Wrap::Wrap>
            {assets.drain(first_item..last_item).map(|x| get_items(x)).collect::<Html>()}
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


    }
}

fn get_asset_city_select(page: &PropertyPage) -> Html {
    html! {

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

    }
}

fn get_items(asset: Html) -> Html {
    html! {
        <Item layouts=vec!(ItemLayout::ItXl(4))>{asset}</Item>
    }
}
