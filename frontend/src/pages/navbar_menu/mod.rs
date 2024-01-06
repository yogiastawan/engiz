use yew::{function_component, html, AttrValue, Html};

use crate::{
    components::menu_link::MenuLink,
    pages::navbar_menu::icon_navbar::{
        IconAbout, IconAhu, IconChiller, IconHelp, IconLogin, IconProfile,
    },
    Route, ScreenBreak,
};

pub mod icon_navbar;

#[function_component(MenuLogin)]
pub(crate) fn menu_login() -> Html {
    html! {
        <MenuLink<Route> link={"/login"} icon={html!{<IconLogin/>}}/>
    }
}

#[function_component(MenuAbout)]
pub(crate) fn menu_about() -> Html {
    html! {
        <MenuLink<Route> route={Route::About} icon={html!{<IconAbout/>}} label={Some("About")} show_label_on={ScreenBreak::Custom(AttrValue::Static("min-[880px]:inline"))} show_on={ScreenBreak::Custom(AttrValue::Static("min-[624px]:flex"))}/>
    }
}

#[function_component(MenuAhu)]
pub(crate) fn menu_ahu() -> Html {
    html! {
        <MenuLink<Route> link={"http://tsmahu.hopto.org:4061"} icon={html!{<IconAhu/>}} label={Some("Ahu")} show_label_on={ScreenBreak::Custom(AttrValue::Static("min-[624px]:inline"))} show_on={ScreenBreak::Small}/>
    }
}

#[function_component(MenuChiller)]
pub(crate) fn menu_chiller() -> Html {
    html! {
        <MenuLink<Route> link={"http://chiller.hopto.org:4061"} icon={html!{<IconChiller/>}} label={Some("Chiller")} show_label_on={ScreenBreak::Custom(AttrValue::Static("min-[624px]:inline"))} show_on={ScreenBreak::Small}/>
    }
}

#[function_component(MenuProfile)]
pub(crate) fn menu_profile() -> Html {
    html! {
        <MenuLink<Route> route={Route::Profile { id: 0 }} icon={html!{<IconProfile/>}} label={Some("Profile")} show_label_on={ScreenBreak::Medium}/>
    }
}

#[function_component(MenuHelp)]
pub(crate) fn menu_help() -> Html {
    html! {
        <MenuLink<Route> route={Route::Help} icon={html!{<IconHelp/>}} label={Some("Help")} show_on={ScreenBreak::Medium} show_label_on={ScreenBreak::Large}/>
    }
}
