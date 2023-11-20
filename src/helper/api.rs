use std::fmt::Debug;

use reqwest::{header::HeaderValue, Client, Method};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub async fn call_api<T>(
    some_struct: &mut T,
    method: Method,
    api_endpoint: &str,
    token: &str,
) -> Result<(), reqwest::Error>
where
    T: DeserializeOwned + Debug,
{
    let base_api = format!("https://api.spacetraders.io/v2{api_endpoint}");

    *some_struct = Client::new()
        .request(method, base_api)
        .header("Authorization", token)
        .header("Content-Length", HeaderValue::from_static("0"))
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}

// pub async fn new_call_api<T>(
//     some_struct: &mut T,
//     method: Method,
//     api_endpoint: &str,
//     token: &str,
// ) -> Result<(), reqwest::Error>
// where
//     T: DeserializeOwned + Debug,
// {
//     let base_api = format!("https://api.spacetraders.io/v2{api_endpoint}");

//     let value: Value = Client::new()
//         .request(method, base_api)
//         .header("Authorization", token)
//         .header("Content-Length", HeaderValue::from_static("0"))
//         .send()
//         .await?
//         .json()
//         .await?;

//     match value["error"] {
//         Value::Null => todo!(),
//     }

//     Ok(())
// }

pub async fn call_api_get(
    api_endpoint: &str,
    token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let base_api = format!("https://api.spacetraders.io/v2{api_endpoint}");

    Client::new()
        .get(base_api)
        .header("Authorization", token)
        .send()
        .await
}

pub async fn call_api_post(
    api_endpoint: &str,
    token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let base_api = format!("https://api.spacetraders.io/v2{api_endpoint}");

    Client::new()
        .post(base_api)
        .header("Authorization", token)
        .header("Content-Length", HeaderValue::from_static("0"))
        .send()
        .await
}

pub async fn call_api_get_generic<'a, T>(
    some_struct: &'a mut T,
    api_endpoint: &str,
    token: &str,
) -> Result<&'a T, reqwest::Error>
where
    T: DeserializeOwned + Debug,
{
    let base_api = format!("https://api.spacetraders.io/v2{api_endpoint}");

    *some_struct = Client::new()
        .get(base_api)
        .header("Authorization", token)
        .send()
        .await?
        .json()
        .await?;

    Ok(some_struct)
}
