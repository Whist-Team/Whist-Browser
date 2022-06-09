use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct ServerConnection {
    url: String,
}

impl ServerConnection {
    pub fn new(url: String) -> ServerConnection {
        ServerConnection { url }
    }
    /// Requests a GET route and transforms it to a JSON object.
    /// # Arguments
    /// * 'url' - The full uri to the requested route include optional path variables.
    pub async fn get_json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(reqwest::get(&self.url).await?.json::<T>().await?)
    }

    pub async fn post_json<D: Serialize>(&self, data: D) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let response = client.post(&self.url).json(&data).send().await?;
        Ok(response)
    }
}
