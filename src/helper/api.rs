use reqwest::{header::HeaderValue, Client};

pub async fn call_api_get(api: &str, token: &str) -> Result<reqwest::Response, reqwest::Error> {
    let base_api = format!("https://api.spacetraders.io/v2{api}");

    Client::new()
        .get(base_api)
        .header("Authorization", token)
        .send()
        .await
}

pub async fn call_api_post(api: &str, token: &str) -> Result<reqwest::Response, reqwest::Error> {
    let base_api = format!("https://api.spacetraders.io/v2{api}");

    Client::new()
        .post(base_api)
        .header("Authorization", token)
        .header("Content-Length", HeaderValue::from_static("0"))
        .send()
        .await
}
