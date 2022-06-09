use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct ServerConnection {
    base_url: String,
}

impl ServerConnection {
    pub fn new(base_url: String) -> ServerConnection {
        ServerConnection { base_url }
    }
    /// Requests a GET route and transforms it to a JSON object.
    /// # Arguments
    /// * 'url' - The full uri to the requested route include optional path variables.
    pub async fn get_json<T: DeserializeOwned>(&self, route: &str) -> Result<T, Error> {
        Ok(reqwest::get(self.base_url.clone() + route)
            .await?
            .json::<T>()
            .await?)
    }

    pub async fn post_json<D: Serialize>(&self, route: &str, data: D) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(self.base_url.clone() + route)
            .json(&data)
            .send()
            .await?;
        Ok(response)
    }
}
