pub mod api;
pub mod pages;

use pages::about::About;
use yew::{function_component, html, AttrValue, Html, Properties, Suspense};
use yew_router::{
    components::Link,
    history::{AnyHistory, History, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

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
    let fallback = html! {<div>{"Loading..."}</div>};
    match route {
        Route::About => html! {
            <About/>
        },
        Route::NotFound => html! {
            <>
            <Suspense {fallback}>
                <h1>{"404 Not found"}</h1>
                <Link<Route> to={Route::Profile { id: 101 }} >{"Go to profile"}</Link<Route>>
            </Suspense>
            </>
        },
        Route::Login => html! {
            <>
            <Suspense {fallback}>
                <div>{"This is login page"}</div>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
                <Link<Route> to={Route::Profile { id: 100 }} >{"Go to profile"}</Link<Route>>
            </Suspense>
            </>
        },
        Route::Profile { id } => html! {
            <>
            <Suspense {fallback}>
                <div>{format!("This is profile of {}",id)}</div>
                <Link<Route> to={Route::Login} >{"Go to Login"}</Link<Route>>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
            </Suspense>
            </>
        },
        Route::Index => html! {
            <>
            <Suspense {fallback}>
                <div>{"HOME"}</div>
                <Link<Route> to={Route::Login} >{"Go to Login"}</Link<Route>>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
            </Suspense>
            </>
        },
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
