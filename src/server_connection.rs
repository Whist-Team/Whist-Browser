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

#[cfg(test)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::response::whist_info::{WhistInfo, WhistInfoFactory};
    use crate::server_connection::ServerConnection;

    #[tokio::test]
    async fn test_get_json() {
        let expected_info =
            WhistInfoFactory::new_info(String::from("whist"), String::from("0.1.0"));

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.clone()))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri() + "/route");
        let response_json = conn.get_json::<WhistInfo>("/route").await.unwrap();
        assert_eq!(response_json, expected_info);
    }

    #[tokio::test]
    async fn test_post_json_without_response_body() {
        let expected_info =
            WhistInfoFactory::new_info(String::from("whist"), String::from("0.1.0"));

        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri() + "/route");
        let response_json = conn
            .post_json::<WhistInfo>("/route", expected_info)
            .await
            .unwrap();
        assert_eq!(response_json.status(), 200);
    }
}
