use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Provides basic REST communication with the server.
pub struct ServerConnection {
    base_url: String,
}

impl ServerConnection {
    /// Constructor for creating a new Server Connection.
    /// # Arguments
    /// * 'base_url' the url of the server
    pub fn new(base_url: String) -> ServerConnection {
        ServerConnection { base_url }
    }
    /// Requests a GET route and transforms it to a JSON object.
    /// # Arguments
    /// * 'route' - The route after the base url from above include optional path variables.
    pub async fn get_json<T: DeserializeOwned>(&self, route: &str) -> Result<T, Error> {
        Ok(reqwest::get(self.base_url.clone() + route)
            .await?
            .json::<T>()
            .await?)
    }

    /// Posts a JSON object to the whist server. It returns the full response which must be
    /// processed somewhere else.
    /// # Arguments
    /// * 'route' - The route after the base url from above include optional path variables.
    /// * 'data' - A serializable object to be send to the server.
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
