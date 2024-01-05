pub mod api;
pub mod components;
pub mod pages;

use pages::about::About;
use yew::{function_component, html, AttrValue, Html, Properties};
use yew_router::{
    components::Link,
    history::{AnyHistory, History, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

use crate::{
    components::nav_bar::NavBar,
    pages::navbar_menu::{MenuAbout, MenuAhu, MenuChiller, MenuLogin, MenuProfile},
};

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
enum Route {
    #[at("/")]
    Index,
    #[at("/login")]
    Login,
    #[at("/profile/:id")]
    Profile { id: usize },
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    let menu = vec![
        html! {<MenuAhu/>},
        html! {<MenuChiller/>},
        html! {<MenuProfile/>},
        html! {<MenuAbout/>},
    ];
    html! {
    <>
    <NavBar name_web={"Eng TSM Z"} active_menu={true} logo={"https://avatars.githubusercontent.com/u/17867264?v=4"} menu={menu}/>

    {match route {
        Route::About => html! {
            <About/>
        },
        Route::NotFound => html! {
            <>
                <h1>{"404 Not found"}</h1>
                <Link<Route> to={Route::Profile { id: 101 }} >{"Go to profile"}</Link<Route>>
            </>
        },
        Route::Login => html! {
            <>
                <div>{"This is login page"}</div>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
                <Link<Route> to={Route::Profile { id: 100 }} >{"Go to profile"}</Link<Route>>
            </>

        },
        Route::Profile { id } => html! {

            <>
                <div>{format!("This is profile of {}",id)}</div>
                <Link<Route> to={Route::Login} >{"Go to Login"}</Link<Route>>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
            </>

        },
        Route::Index => html! {
            <>
                <div>{"HOME"}</div>
                <Link<Route> to={Route::Login} >{"Go to Login"}</Link<Route>>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
            </>
        },
    }}
    </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route> render={switch}/>
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
