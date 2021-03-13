use crate::global::{DEFAULT_UX_ASSET_COLOR, DEFAULT_UX_ASSET_SIZE};

use yew::prelude::*;
use yew_assets::business_assets::{BusinessAssets, BusinessIcon};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yew_styles::text::{Text, TextType};
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct HomePage {
    props: Props,
}

impl Component for HomePage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!("Called change");
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
          <Container direction=Direction::Row wrap=Wrap::Wrap>
            <Item layouts=vec!(ItemLayout::ItXs(12))>

              <Text
                text_type=TextType::Paragraph
                text_size=Size::Medium
                plain_text=""
                html_text=Some(
                  html!{
                    <>
                      {"Shylock te ayuda a buscar las mejores subastas. Busca "}
                      <BusinessAssets
                        icon=BusinessIcon::Target
                        fill=DEFAULT_UX_ASSET_COLOR
                        size=(DEFAULT_UX_ASSET_SIZE.to_string(), DEFAULT_UX_ASSET_SIZE.to_string()) />
                      {" para bienes que merezcan la pena."}
                    </>
                }) />
            </Item>
          </Container>
        }
    }
}
