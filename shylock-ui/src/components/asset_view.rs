use crate::global::{DEFAULT_UX_ASSET_COLOR, DEFAULT_UX_ASSET_SIZE};
use crate::routes::{OtherDetailPage, PropertyDetailPage, VehicleDetailPage};
use crate::utils::{format_valuation, get_bidinfo, is_targeted_asset, summarize};

use shylock_data::{types::Asset, Other, Property, Vehicle};
use wasm_bindgen::JsCast;
use yew::{prelude::*, utils::document, web_sys::HtmlElement};
use yew_assets::business_assets::{BusinessAssets, BusinessIcon};
use yew_assets::editing_assets::{EditingAssets, EditingIcon};
use yew_styles::button::Button;
use yew_styles::card::Card;
use yew_styles::modal::Modal;
use yew_styles::styles::{Palette, Size, Style};
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub position: usize,
    pub asset: &'static Asset,
}

pub struct AssetView {
    props: Props,
    current_modal: usize,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CloseModal,
    OpenModal(usize),
    CloseModalByKb(KeyboardEvent),
}

impl Component for AssetView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            current_modal: std::usize::MAX,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let body_style = document()
            .body()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .style();

        match msg {
            Msg::CloseModal => {
                body_style.set_property("overflow", "auto").unwrap();
                self.current_modal = std::usize::MAX;
            }
            Msg::CloseModalByKb(keyboard_event) => {
                if keyboard_event.key_code() == 27 {
                    body_style.set_property("overflow", "auto").unwrap();
                    self.current_modal = std::usize::MAX;
                }
            }
            Msg::OpenModal(index) => {
                body_style.set_property("overflow", "hidden").unwrap();

                self.current_modal = index;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let modal_position = self.props.position;
        match self.props.asset {
            Asset::Property(property) => {
                html! {
                    <>
                    <Card
                      key=self.props.position.to_string()
                      card_size=Size::Medium
                      card_palette=Palette::Clean
                      card_style=Style::Outline
                      class_name="pointer"
                      onclick_signal=self.link.callback(move |_| Msg::OpenModal(modal_position))
                      header=Some(self.get_property_header(property))
                      body=Some(html!{<div>{summarize(&property.description)}</div>})
                      footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&property.bidinfo, &property.auction_id).value)}{" €"}</b></div>}) />
                      {self.get_modal(html!{ <PropertyDetailPage position=self.props.position /> })}
                    </>
                }
            }
            Asset::Vehicle(vehicle) => {
                html! {
                <>
                  <Card
                    key=self.props.position.to_string()
                    card_size=Size::Medium
                    card_palette=Palette::Clean
                    card_style=Style::Outline
                    class_name="pointer"
                    onclick_signal=self.link.callback(move |_| Msg::OpenModal(modal_position))
                    header=Some(self.get_vehicle_header(vehicle))
                    body=Some(html!{<div>{summarize(&vehicle.description)}</div>})
                    footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&vehicle.bidinfo, &vehicle.auction_id).value)}{" €"}</b></div>}) />
                    {self.get_modal(html!{ <VehicleDetailPage position=self.props.position /> })}
                </>
                }
            }
            Asset::Other(other) => {
                html! {
                <>
                  <Card
                    key=self.props.position.to_string()
                    card_size=Size::Medium
                    card_palette=Palette::Clean
                    card_style=Style::Outline
                    class_name="pointer"
                    onclick_signal=self.link.callback(move |_| Msg::OpenModal(modal_position))
                    header=Some(self.get_other_header(other))
                    body=Some(html!{<div>{summarize(&other.description)}</div>})
                    footer=Some(html!{<div><b>{format_valuation(&get_bidinfo(&other.bidinfo, &other.auction_id).value)}{" €"}</b></div>}) />
                    {self.get_modal(html!{ <OtherDetailPage position=self.props.position /> })}
                </>
                }
            }
        }
    }
}

impl AssetView {
    fn get_modal(&self, body: Html) -> Html {
        html! {
        <Modal
          header=html!{
           <Button
             button_palette=Palette::Clean
             button_style=Style::Outline
             onclick_signal=self.link.callback(|_| Msg::CloseModal)>
              <EditingAssets
                icon=EditingIcon::XCircle
                fill=DEFAULT_UX_ASSET_COLOR
                size=(DEFAULT_UX_ASSET_SIZE.to_string(), DEFAULT_UX_ASSET_SIZE.to_string()) />
            </Button>
          }
          header_palette=Palette::Standard
          modal_palette=Palette::Standard
          modal_size=Size::Big
          body=body
          body_style=Style::Outline
          body_palette=Palette::Standard
          is_open={self.current_modal == self.props.position}
          onclick_signal= self.link.callback(|_| Msg::CloseModal)
          onkeydown_signal= self.link.callback(|e| Msg::CloseModalByKb(e)) />
        }
    }

    fn get_property_header(&self, property: &Property) -> Html {
        if is_targeted_asset(&property.bidinfo, &property.auction_id) {
            html! {
                <>
                  <BusinessAssets
                    icon=BusinessIcon::Target
                    fill=DEFAULT_UX_ASSET_COLOR
                    size=(DEFAULT_UX_ASSET_SIZE.to_string(), DEFAULT_UX_ASSET_SIZE.to_string()) />
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
                    icon=BusinessIcon::Target
                    fill=DEFAULT_UX_ASSET_COLOR
                    size=(DEFAULT_UX_ASSET_SIZE.to_string(), DEFAULT_UX_ASSET_SIZE.to_string()) />
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
                    icon=BusinessIcon::Target
                    fill=DEFAULT_UX_ASSET_COLOR
                    size=(DEFAULT_UX_ASSET_SIZE.to_string(), DEFAULT_UX_ASSET_SIZE.to_string()) />
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
