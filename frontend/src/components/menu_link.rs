use yew::{classes, function_component, html, AttrValue, Html, Properties};
use yew_router::{components::Link, Routable};

use crate::ScreenBreak;

#[derive(Clone, PartialEq, Properties)]
pub struct PropsMenuLink<T: PartialEq + Clone + Routable + 'static> {
    #[prop_or_default]
    pub link: Option<AttrValue>,
    #[prop_or_default]
    pub route: Option<T>,
    pub icon: Html,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or(ScreenBreak::AllSize)]
    pub show_on: ScreenBreak,
    #[prop_or(ScreenBreak::AllSize)]
    pub show_label_on: ScreenBreak,
}

#[function_component(MenuLink)]
pub fn menu_link<T: PartialEq + Clone + Routable + 'static>(props: &PropsMenuLink<T>) -> Html {
    let label_class = match props.show_label_on.clone() {
        ScreenBreak::AllSize => classes!("inline"),
        ScreenBreak::ExtraSmall => classes!("hidden", "xs:inline"),
        ScreenBreak::Small => classes!("hidden", "sm:inline"),
        ScreenBreak::Medium => classes!("hidden", "md:inline"),
        ScreenBreak::Large => classes!("hidden", "lg:inline"),
        ScreenBreak::ExtraLarge => classes!("hidden", "xl:inline"),
        ScreenBreak::Custom(x) => classes!("hidden", x),
    };

    let m_class = match props.show_on.clone() {
        ScreenBreak::AllSize => classes!("flex"),
        ScreenBreak::ExtraSmall => classes!("hidden", "xs:flex"),
        ScreenBreak::Small => classes!("hidden", "sm:flex"),
        ScreenBreak::Medium => classes!("hidden", "md:flex"),
        ScreenBreak::Large => classes!("hidden", "lg:flex"),
        ScreenBreak::ExtraLarge => classes!("hidden", "xl:flex"),
        ScreenBreak::Custom(x) => classes!("hidden", x),
    };
    match props.route.clone() {
        Some(x) => {
            html! {
                <Link<T> classes={classes!("text-gray-400", "py-3", "px-2", "hover:text-gray-200", "gap-1", "items-center",m_class)} to={x}>
                    {props.icon.clone()} <span class={classes!(label_class)}>{props.label.clone()}</span>
                </Link<T>>
            }
        }
        None => {
            html! {
                <a class={classes!("text-gray-400", "py-3", "px-2", "hover:text-gray-200", "gap-1", "items-center",m_class)} href={props.link.clone()}>
                    {props.icon.clone()} <span class={classes!(label_class)}>{props.label.clone()}</span>
                </a>
            }
        }
    }
}
