use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_styles::layouts::container::{JustifyContent, Mode};
use yew_styles::{
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Style},
};

pub const ITEMS_PER_PAGE: usize = 100;

/// Pagination component
pub struct Pagination {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub items_count: usize,
    pub current_page: usize,
    pub callback: Callback<usize>,
}

pub enum Msg {
    PaginationChanged(usize),
}

impl Component for Pagination {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Pagination { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PaginationChanged(page) => {
                self.props.callback.emit(page);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.items_count < ITEMS_PER_PAGE {
            return html! {};
        }

        html! {
            <Navbar class_name="navbar-router" fixed=Fixed::None navbar_style=Style::Outline navbar_palette=Palette::Clean>
            <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
            { self.get_navbar_items() }
            </NavbarContainer>
            </Navbar>
        }
    }
}

impl Pagination {
    fn get_navbar_items(&self) -> Html {
        // Calculate page numbers
        let max_page = (self.props.items_count as f32 / ITEMS_PER_PAGE as f32).ceil() as usize;
        let mut pages: Vec<(usize, Callback<MouseEvent>)> = vec![];
        for page in 0..max_page {
            pages.push((
                page,
                self.link.callback(move |ev: MouseEvent| {
                    ev.prevent_default();
                    Msg::PaginationChanged(page)
                }),
            ));
        }
        pages
            .drain(..)
            .map(|page| {
                html! {
                  <NavbarItem
                    active={page.0 == self.props.current_page}
                    onclick_signal=page.1>
                    <span>{page.0 + 1}</span>
                  </NavbarItem>
                }
            })
            .collect::<Html>()
    }
}
