use yew::{function_component, html, AttrValue, Html, Properties};

pub mod contributor_card;
pub mod icon;
pub mod menu_link;
pub mod nav_bar;

#[derive(Clone, PartialEq, Properties)]
pub struct ContProps {
    pub name: AttrValue,
    pub link: Option<AttrValue>,
    pub link_type: AttrValue,
    pub img: AttrValue,
}

#[function_component(ContributorCard)]
pub fn contrib_card(p: &ContProps) -> Html {
    html! {
        <div></div>
    }
}
