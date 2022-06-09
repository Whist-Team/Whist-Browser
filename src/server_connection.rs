use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Requests a GET route and transforms it to a JSON object.
/// # Arguments
/// * 'url' - The full uri to the requested route include optional path variables.
pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    Ok(reqwest::get(url).await?.json::<T>().await?)
}

pub async fn post_json<D: Serialize>(url: &str, data: D) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let response = client.post(url).json(&data).send().await?;
    Ok(response)
}