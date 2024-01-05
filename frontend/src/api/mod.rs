use common::{ApiResponse, BASE_URL, ApiAboutRespons};

pub async fn get_api(
    url: &str,
    is_relative: bool,
) -> Result<ApiResponse<ApiAboutRespons>, String> {
    let url = match is_relative {
        true => BASE_URL.to_owned() + "/capi/about",
        false => url.to_string(),
    };
    let resp = reqwest::get(url).await;
    let resp = match resp {
        Ok(x) => x,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let data = resp.json::<ApiResponse<ApiAboutRespons>>().await.unwrap();
    Ok(data)
}
