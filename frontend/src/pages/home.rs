// use log::debug;
use yew::{function_component, html, Callback, Html, HtmlResult, Suspense};


#[function_component(Content)]
fn content() -> HtmlResult {
    Ok(html!())
}

#[function_component(Home)]
pub fn home() -> Html {
    let fallback = html! {<div>{"Loading... Loading... Loading... Loading... Loading..."}</div>};

    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        // debug!("{}", greeting); // if uncommented will print
    });

    html! {
        <Suspense {fallback}>
            <div>{"HOME"}</div>
            <button {onclick}>{"Click me to "}</button>
        </Suspense>
    }
}
