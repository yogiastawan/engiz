use yew::{function_component, html, Html, Properties, AttrValue};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub logo: AttrValue,
    pub menu: Vec<Html>,
    pub name_web: String,
    pub active_menu: bool,
}

#[function_component]
pub fn NavBar(props: &Props) -> Html {
    let logo = props.logo.clone();
    let menu = props.menu.clone();
    let name_web = props.name_web.clone();
    let active_menu = props.active_menu.clone();

    html! {
        <>
        <header>
        <nav class="fixed left-0 right-0 top-0 z-[1] py-3 px-4 bg-slate-800 font-semibold transition-all duration-300"
        id="nav-bar">
        <div class="flex justify-between items-center w-auto min-[1920px]:w-[1888px] mx-0 min-[1920px]:mx-auto">
        <div class="flex gap-4 items-center" id="nav">
           if active_menu{
            <button class="block md:block xl:hidden text-gray-400 py-3 px-2 hover:text-gray-200"
                id="nav-bar-btn-menu-draw">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2"
                    stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <line x1="4" y1="6" x2="20" y2="6"></line>
                    <line x1="4" y1="12" x2="20" y2="12"></line>
                    <line x1="4" y1="18" x2="20" y2="18"></line>
                </svg>
            </button>
            }
            <a class="flex md:flex gap-1 items-center text-white" id="nav-bar-logo-link" href="/">
                <img class="xs:block" id="logo-img" width="50" src={logo.clone()}
                    alt={name_web.clone()}/>
                <p class="duration-300 hover:text-gray-400">{name_web}</p>
            </a>
        </div>
        <div class="flex gap-4">
            {menu.into_iter().collect::<Html>()}
        </div>
        </div>
        </nav>

        </header>
        </>
    }
}
