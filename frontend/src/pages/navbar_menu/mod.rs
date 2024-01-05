use yew::{function_component, html, Html};

use crate::{
    components::menu_link::MenuLink,
    pages::navbar_menu::icon_navbar::{IconAbout, IconAhu, IconChiller, IconLogin, IconProfile},
};

pub mod icon_navbar;

#[function_component(MenuLogin)]
pub(crate) fn menu_login() -> Html {
    html! {
        <MenuLink link={"/login"} icon={html!{<IconLogin/>}}/>
    }
}

#[function_component(MenuAbout)]
pub(crate) fn menu_about() -> Html {
    html! {
        <MenuLink link={"/about"} icon={html!{<IconAbout/>}}/>
    }
}

#[function_component(MenuAhu)]
pub(crate) fn menu_ahu() -> Html {
    html! {
        <MenuLink link={"http://tsmahu.hopto.org:4061"} icon={html!{<IconAhu/>}}/>
    }
}

#[function_component(MenuChiller)]
pub(crate) fn menu_chiller() -> Html {
    html! {
        <MenuLink link={"http://chiller.hopto.org:4061"} icon={html!{<IconChiller/>}}/>
    }
}

#[function_component(MenuProfile)]
pub(crate) fn menu_profile() -> Html {
    html! {
        <MenuLink link={"/profile"} icon={html!{<IconProfile/>}}/>
    }
}
