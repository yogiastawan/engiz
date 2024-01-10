use std::rc::Rc;

use common::{ApiAboutRespons, ApiResponse};
use yew::{function_component, html, use_prepared_state, Html, HtmlResult, Properties, Suspense};

use super::navbar_menu::PageProps;

#[derive(Clone, PartialEq, Properties)]
pub struct AboutProps {
    #[prop_or_default]
    pub props: Option<ApiResponse<ApiAboutRespons>>,
}

#[function_component]
fn Content(props: &AboutProps) -> HtmlResult {
    let resp = use_prepared_state!((), async move |_| -> Result<
        ApiResponse<ApiAboutRespons>,
        String,
    > {
        use crate::api::get_api;
        use common::API_C;

        let resp = get_api("/about", API_C).await;
        let resp = match resp {
            Ok(x) => x,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let data = resp.json::<ApiResponse<ApiAboutRespons>>().await;
        let data = match data {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        Ok(data)
    })?;

    let rs = match resp {
        Some(x) => x,
        None => match props.props.clone() {
            Some(x) => Rc::new(Ok(x)),
            None => {
                return Ok(html! {
                    <div>{"No data fetched..."}</div>
                });
            }
        },
    };

    let data = match rs.as_ref() {
        Ok(x) => x,
        Err(e) => {
            return Ok(html! {
                <>
                <div>{"error: "}{e}</div>
                </>
            })
        }
    };
    let about = data.content.clone();

    Ok(html! {
    <>
        <div class="flex flex-col items-center max-w-screen-md mx-auto">
            <div class="rounded-2xl relative group cursor-default">
                <div class="left-0 right-0 top-0 bottom-0 shadow-none group-hover:shadow-sm bg-gradient-to-br blur-sm group-hover:blur-md group-hover:bg-gradient-to-tl from-cyan-500 to-blue-500 rounded-2xl absolute transition-all duration-1000">
                </div>
                <div class="text-lg rounded-2xl blur-none font-bold text-center group-hover:dark:text-slate-100 dark:text-gray-300 p-4 transition-all duration-1000">
                    <p>{about.name.clone()}</p>
                    <p>{"v"}{about.version.clone()}</p>
                </div>
            </div>
            <div class="mt-3 p-4 relative w-full group cursor-default">
                <div class="left-4 right-4 bottom-0 blur-none group-hover:left-0 group-hover:right-0 h-[2px] rounded-[1px] group-hover:h-[4px] group-hover:rounded-[2px] absolute bg-gradient-to-r group-hover:bg-gradient-to-l from-cyan-500 via-indigo-500 to-blue-500 transition-all duration-1000"></div>
                <div class="text-center group-hover:dark:text-slate-100 transition-all duration-1000 blur-none">{about.description.clone()}</div>
            </div>
            <div class="mt-6 p-4 pb-2 w-full relative group">
                <div class="left-4 right-4 bottom-0 blur-none group-hover:left-0 group-hover:right-0 h-[2px] rounded-[1px] group-hover:h-[4px] group-hover:rounded-[2px] absolute bg-gradient-to-r group-hover:bg-gradient-to-l from-cyan-500 via-indigo-500 to-blue-500 transition-all duration-1000"></div>
                <div>
                    <div class="text-lg font-bold group-hover:dark:text-slate-100 text-center transition-all duration-1000">{"Contributors"}</div>
                    <div class="text-center text-sm group-hover:dark:text-slate-100 transition-all duration-1000">{"People who have worked in developing this application"}</div>
                    <div class="mt-3 flex justify-around flex-wrap">{"list contributor"}</div>
                </div>
            </div>
            <div class="mt-6 p-4 pb-2 w-full relative group">
                <div class="left-4 right-4 bottom-0 blur-none group-hover:left-0 group-hover:right-0 h-[2px] rounded-[1px] group-hover:h-[4px] group-hover:rounded-[2px] absolute bg-gradient-to-r group-hover:bg-gradient-to-l from-cyan-500 via-indigo-500 to-blue-500 transition-all duration-1000"></div>
                <div>
                    <div class="text-lg font-bold group-hover:dark:text-slate-100 text-center transition-all duration-1000">{"Built With"}</div>
                    <div class="text-center text-sm group-hover:dark:text-slate-100 transition-all duration-1000">{"libraries used to develop this application"}</div>
                    <div class="mt-3 flex justify-around flex-wrap">{"list library"}</div>
                </div>
            </div>

            <div class="mt-6 p-4 pb-2 w-full relative group">
                <div class="left-4 right-4 bottom-0 blur-none group-hover:left-0 group-hover:right-0 h-[2px] rounded-[1px] group-hover:h-[4px] group-hover:rounded-[2px] absolute bg-gradient-to-r group-hover:bg-gradient-to-l from-cyan-500 via-indigo-500 to-blue-500 transition-all duration-1000"></div>
                <div>
                    <div class="text-lg font-bold group-hover:dark:text-slate-100 text-center transition-all duration-1000">{"Contact Us"}</div>
                    <div class="text-center text-sm group-hover:dark:text-slate-100 transition-all duration-1000">{"If you find a bug, want to request a feature, or need help using this app, please contact us"}</div>
                    <div class="mt-3 flex justify-around flex-wrap">{"list contact"}</div>
                </div>
            </div>
        </div>
    </>
    })
}

#[function_component(AboutPage)]
pub fn about_page(props: &PageProps<ApiAboutRespons>) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content props={props.props.clone()}/>
        </Suspense>
    }
}
