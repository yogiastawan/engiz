use yew::{classes, function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};
use yew_router::{components::Link, Routable};

use crate::ScreenBreak;

#[derive(Clone, PartialEq, Properties)]
pub struct PropsMenuLink {
    #[prop_or_default]
    pub link: AttrValue,
    pub icon: Html,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or(ScreenBreak::AllSize)]
    pub show_on: ScreenBreak,
    #[prop_or(ScreenBreak::AllSize)]
    pub show_label_on: ScreenBreak,
    #[prop_or_default]
    pub callback: Option<Callback<AttrValue>>,
}

#[function_component(MenuLink)]
pub fn menu_link<T: PartialEq + Clone + Routable + 'static>(props: &PropsMenuLink) -> Html {
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

    let link = props.link.clone();
    let link_route = T::recognize(link.clone().as_ref());

    match props.callback.clone() {
        Some(x) => {
            let a = link.clone();
            let onclick = x.reform(move |e: MouseEvent| {
                e.prevent_default();

                a.clone().into()
            });
            html! {
                <a {onclick} class={classes!("transition-all","duration-300","text-gray-700","dark:text-gray-400", "py-3", "px-2", "hover:text-sky-700","dark:hover:text-gray-200", "gap-1", "items-center",m_class)} href={link.clone()}>
                    {props.icon.clone()} <span class={classes!(label_class.clone())}>{props.label.clone()}</span>
                </a>
            }
        }
        None => {
            let route = match link_route {
                Some(x) => x,
                None => {
                    return html! {
                        <a  class={classes!("transition-all","duration-300","text-gray-700","dark:text-gray-400", "py-3", "px-2", "hover:text-sky-700","dark:hover:text-gray-200", "gap-1", "items-center",m_class)} href={link.clone()}>
                            {props.icon.clone()} <span class={classes!(label_class.clone())}>{props.label.clone()}</span>
                        </a>
                    }
                }
            };

            match route.to_path() == "/404" {
                true => {
                    html! {
                        <a  class={classes!("transition-all","duration-300","text-gray-700","dark:text-gray-400", "py-3", "px-2", "hover:text-sky-700","dark:hover:text-gray-200", "gap-1", "items-center",m_class)} href={link.clone()}>
                            {props.icon.clone()} <span class={classes!(label_class.clone())}>{props.label.clone()}</span>
                        </a>
                    }
                }
                false => {
                    html! {
                        <Link<T> classes={classes!("transition-all","duration-300","text-gray-700","dark:text-gray-400", "py-3", "px-2", "hover:text-sky-700","dark:hover:text-gray-200", "gap-1", "items-center",m_class)} to={route}>
                            {props.icon.clone()} <span class={classes!(label_class.clone())}>{props.label.clone()}</span>
                        </Link<T>>
                    }
                }
            }
        }
    }
}
