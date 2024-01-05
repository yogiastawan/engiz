use common::{ApiAboutRespons, ApiResponse};
use yew::prelude::*;

use crate::api::get_api;

#[function_component]
fn Content() -> HtmlResult {
    let resp = use_prepared_state!((), async move |_| -> Result<
        ApiResponse<ApiAboutRespons>,
        String,
    > { get_api("/capi/about/", true).await })?;

    let resp = match resp {
        Some(x) => x,
        None => {
            return Ok(html! {
                <div>{"error: Cannot Connect to server"}</div>
            })
        }
    };

    let data = match resp.as_ref() {
        Ok(x) => x,
        Err(e) => {
            return Ok(html! {
                <div>{"error: "}{e}</div>
            })
        }
    };
    Ok(html! {
        <div>{"Random UUID: "}{data.content.description.clone()}</div>
    })
}

#[function_component]
pub fn About() -> Html {
    let fallback = html! {<div>{"Loading... Loading... Loading... Loading... Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
