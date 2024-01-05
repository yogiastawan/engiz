use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub link: AttrValue,
    pub icon: Html,
}

#[function_component(MenuLink)]
pub fn menu_link(props: &Props) -> Html {
    html! {
    <a class="md:block text-gray-400 py-3 px-2 hover:text-gray-200" href={props.link.clone()}>
        {props.icon.clone()}
    </a>
    }
}
