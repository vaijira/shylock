use crate::global::{ASSETS, DEFAULT_UX_ASSET_COLOR, DEFAULT_UX_ASSET_SIZE};
use crate::utils::{format_valuation, get_bidinfo, get_external_url};

use shylock_data::types::Asset;
use yew::prelude::*;
use yew_assets::nav_assets::{NavAssets, NavIcon};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yew_styles::text::{Text, TextType};
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub position: usize,
}

pub struct OtherDetailPage {
    props: Props,
}

impl Component for OtherDetailPage {
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
        let asset = ASSETS.get().unwrap().get(self.props.position);
        if let Some(Asset::Other(other)) = asset {
            html! {
              <Container direction=Direction::Row wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(2))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text="Id subasta:"
                    html_text=None />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(10))>
                  <a href={get_external_url(&other.auction_id)}
                    alt="enlace a subastas BOE"
                    target="_blank">
                    {&other.auction_id}{" "}
                    <NavAssets
                      icon=NavIcon::ExternalLink
                      fill=DEFAULT_UX_ASSET_COLOR
                      size=(DEFAULT_UX_ASSET_SIZE.to_string(), DEFAULT_UX_ASSET_SIZE.to_string()) />
                  </a>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(2))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text="Descripción:"
                    html_text=None />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(10))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text=""
                    html_text=html!{&other.description} />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(2))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text="Información adicional:"
                    html_text=None />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(10))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text=""
                    html_text=html!{&other.additional_information} />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(2))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text="Valor subasta:"
                    html_text=None />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(10))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text=""
                    html_text=html!{<>{format_valuation(&get_bidinfo(&other.bidinfo, &other.auction_id).value)}{" €"}</>} />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(2))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text="Cantidad reclamada:"
                    html_text=None />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(10))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text=""
                    html_text=html!{<>{format_valuation(&get_bidinfo(&other.bidinfo, &other.auction_id).claim_quantity)}{" €"}</>} />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(2))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text="Valor tasación:"
                    html_text=None />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(10))>
                  <Text
                    text_type=TextType::Plain
                    text_size=Size::Small
                    plain_text=""
                    html_text=html!{<>{format_valuation(&get_bidinfo(&other.bidinfo, &other.auction_id).appraisal)}{" €"}</>} />
                </Item>
              </Container>
            }
        } else {
            html! { "error "}
        }
    }
}
