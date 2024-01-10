use common::{ApiAboutRespons, ApiResponse, API_C};
use reqwest::Response;
use serde_json::from_str;
use yew::{html, AttrValue, Component, Html, Properties};
use yew_router::{components::Link, scope_ext::RouterScopeExt, Routable};

use crate::{
    components::nav_bar::NavBar,
    pages::{
        about::AboutPage,
        home::Home,
        navbar_menu::{MenuAbout, MenuAhu, MenuChiller, MenuHelp, MenuProfile},
    },
    Route,
};

#[derive(Debug)]
pub enum RequestState<T> {
    NoRequest,
    Requesting,
    Parsing,
    Success(String, Option<T>),
    Failed(String),
}

#[derive(Properties, PartialEq)]
pub struct AppContentProps {
    pub route_page: Route,
    #[prop_or(AttrValue::Static(""))]
    pub page_title: AttrValue,
}

pub enum AppContentMsg {
    SetRequestState(RequestState<Route>),
    GetPage(AttrValue),
    Parse(Response, Option<AttrValue>),
}

pub struct AppContent {
    state: RequestState<Route>,
    // get_content: Callback<AttrValue>,
}

impl AppContent {
    pub(crate) fn routing(route: Route) -> Html {
        match route {
            Route::About => html! {
                <AboutPage/>
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
                    <Home/>
                </>
            },
            Route::Help => html! {
                <>
                <div>{"Help"}</div>
                <Link<Route> to={Route::Login} >{"Go to Login"}</Link<Route>>
                <Link<Route> to={Route::About} >{"Go to about"}</Link<Route>>
            </>
            },
        }
    }

    pub(crate) fn state_render(state: &RequestState<Route>, route: Route) -> Html {
        match state {
            RequestState::NoRequest => {
                html! {Self::routing(route)}
            }
            RequestState::Success(x, r) => {
                let r = match r {
                    Some(x) => x,
                    None => &Route::NotFound,
                };
                match r {
                    Route::Index => {
                        html!()
                    }
                    Route::Login => {
                        html! {}
                    }
                    Route::Profile { id: _ } => {
                        html!()
                    }
                    Route::About => {
                        let a = from_str::<ApiResponse<ApiAboutRespons>>(x.as_str());
                        match a {
                            Ok(x) => {
                                html! {
                                    <AboutPage props={x}/>
                                }
                            }
                            Err(e) => html! {
                                <div>{e.to_string()}</div>
                            },
                        }
                    }
                    Route::Help => {
                        html!()
                    }
                    _ => {
                        html!()
                    }
                }
            }
            RequestState::Failed(x) => {
                html! {
                    <>
                    <div>{x}</div>
                    </>
                }
            }
            RequestState::Requesting => {
                html! {
                    <div>{"requesting..."}</div>
                }
            }
            RequestState::Parsing => {
                html! {
                    <div>{"Parsing content"}</div>
                }
            }
        }
    }
}

impl Component for AppContent {
    type Message = AppContentMsg;

    type Properties = AppContentProps;

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self {
            state: RequestState::NoRequest,
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppContentMsg::SetRequestState(state) => {
                self.state = state;
                true
            }
            AppContentMsg::GetPage(url) => {
                let navigator = ctx.link().navigator().unwrap();
                if let Some(x) = Route::recognize(url.as_str()) {
                    if x.to_path() != "/404" {
                        navigator.push(&x)
                    }
                }

                let url = url.clone();
                ctx.link().send_future(async move {
                    use crate::api::get_api;
                    match get_api(url.as_str(), API_C).await {
                        Ok(x) => AppContentMsg::Parse(x, Some(url)),
                        Err(e) => {
                            AppContentMsg::SetRequestState(RequestState::Failed(e.to_string()))
                        }
                    }
                });
                ctx.link()
                    .send_message(AppContentMsg::SetRequestState(RequestState::Requesting));
                false
            }
            AppContentMsg::Parse(x, r) => {
                ctx.link().send_future(async move {
                    if x.status() != 200 {
                        let url = match r.clone() {
                            Some(x) => format!(" \"{}\"", x),
                            None => "".to_string(),
                        };
                        return AppContentMsg::SetRequestState(RequestState::Failed(format!(
                            "Sorry the are problems when requesting page{}. Error: {}",
                            url,
                            x.status()
                        )));
                    }
                    let route = match r.clone() {
                        Some(y) => Route::recognize(y.as_str()),
                        None => None,
                    };
                    let str = x.text().await;
                    match str {
                        Ok(x) => AppContentMsg::SetRequestState(RequestState::Success(x, route)),
                        Err(e) => {
                            AppContentMsg::SetRequestState(RequestState::Failed(e.to_string()))
                        }
                    }
                });
                ctx.link()
                    .send_message(AppContentMsg::SetRequestState(RequestState::Parsing));
                false
            }
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        let cb_about = ctx.link().callback(AppContentMsg::GetPage);
        let menu = vec![
            html! {<MenuAhu/>},
            html! {<MenuChiller/>},
            html! {<MenuProfile callback={cb_about.clone()}/>},
            html! {<MenuAbout  callback={cb_about.clone()}/>},
            html! {<MenuHelp  callback={cb_about}/>},
        ];
        let route = ctx.props().route_page.clone();
        html! {
            <>
                <NavBar<Route> home_route={Route::Index}
                    name_web={"Eng TSM Z"} active_menu={true}
                    logo={"https://avatars.githubusercontent.com/u/17867264?v=4"} menu={menu}/>
                <section class="py-4 text-gray-800 dark:text-gray-300 mt-20 mx-5 w-auto min-[1920px]:w-[1888px] min-[1920px]:mx-auto">
                {
                    Self::state_render(&self.state, route)
                }
                </section>
            </>
        }
    }
}
