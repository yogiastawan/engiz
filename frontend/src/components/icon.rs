use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    pub path: Vec<Html>,
    #[prop_or(24u32)]
    pub width: u32,
    #[prop_or(24u32)]
    pub height: u32,
    #[prop_or_default]
    pub color: Option<AttrValue>,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width={props.width.to_string()} height={props.height.to_string()} viewBox={format!("0 0 {} {}",props.width.to_string(),props.height.to_string())}
                    fill={match props.color.clone(){Some(x)=>x,None=>"currentColor".to_string().into()}} stroke-linecap="round" stroke-linejoin="round">
                    {
                        props.path.clone().into_iter().collect::<Html>()
                    }
        </svg>
    }
}
