use reqwest::Error;
use serde::de::DeserializeOwned;

pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    Ok(reqwest::get(url)
        .await?
        .json::<T>()
        .await?
    )
}