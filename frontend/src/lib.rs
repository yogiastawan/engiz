pub mod api;
pub mod app_content;
pub mod components;
pub mod pages;

use yew::{function_component, html, AttrValue, Html, Properties};
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

use crate::app_content::AppContent;

#[derive(Debug, Clone)]
pub struct GetPageError {
    pub message: String,
    pub description: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenBreak {
    AllSize,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(AttrValue),
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/login")]
    Login,
    #[at("/profile/:id")]
    Profile { id: usize },
    #[at("/about")]
    About,
    #[at("/help")]
    Help,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    html! {
        <AppContent route_page={route}/>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
       <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);

    html! {
        <Router history={history}>
            <Switch<Route> render={switch}/>
        </Router>
    }
}
