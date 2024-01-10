use common::BASE_URL;
use reqwest::{Error, Response};

pub async fn get_api(url: &str, api_path: &str) -> Result<Response, Error> {
    let url = if url.chars().nth(0) == Some('/') {
        BASE_URL.to_owned() + api_path + url
    } else {
        url.to_string()
    };
    let resp = reqwest::get(&url).await;

    resp
}
