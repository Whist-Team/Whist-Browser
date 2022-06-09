use reqwest::Error;
use serde::de::DeserializeOwned;

/// Requests a GET route and transforms it to a JSON object.
/// # Arguments
/// * 'url' - The full uri to the requested route include optional path variables.
pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    Ok(reqwest::get(url).await?.json::<T>().await?)
}
