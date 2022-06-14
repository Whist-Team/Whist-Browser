use reqwest::{Error, IntoUrl, Method};

use crate::network::*;

/// Service to provide call to whist server routes.
pub struct ServerService {
    server_connection: ServerConnection,
}

impl ServerService {
    /// Constructor
    /// # Arguments
    /// * 'base_url' the url of the server
    pub fn new(base_url: impl IntoUrl) -> Self {
        Self {
            server_connection: ServerConnection::new(base_url),
        }
    }

    /// Retrieves the whist info object from the server.
    pub async fn get_info(&self) -> Result<WhistInfo, Error> {
        self.server_connection
            .request_with_result(Method::GET, "", Option::<&()>::None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::network::*;

    #[tokio::test]
    async fn test_get_json() {
        let expected_info = WhistInfo::new("whist", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.to_owned()))
            .mount(&mock_server)
            .await;
        let service = ServerService::new(mock_server.uri());
        let response_json = service.get_info().await.unwrap();
        assert_eq!(response_json, expected_info);
    }
}
