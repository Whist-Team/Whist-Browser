use reqwest::{Error, IntoUrl};
use crate::transfer::whist_info::WhistInfo;
use crate::server_connection::ServerConnection;

/// Service to provide call to whist server routes.
pub struct ServerService {
    server_connection: ServerConnection,
}

impl ServerService {
    /// Constructor
    /// # Arguments
    /// * 'base_url' the url of the server
    pub fn new<U: IntoUrl>(base_url: U) -> Self {
        Self {
            server_connection: ServerConnection::new(base_url),
        }
    }

    /// Retrieves the whist info object from the server.
    pub async fn get_info(&self) -> Result<WhistInfo, Error> {
        self.server_connection.get_json("").await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::method;
    use crate::response::whist_info::WhistInfo;
    use crate::server_service::ServerService;

    #[tokio::test]
    async fn test_get_json() {
        let expected_info = WhistInfo::new("whist".to_string(), "0.1.0".to_string());

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.clone()))
            .mount(&mock_server)
            .await;
        let service = ServerService::new(mock_server.uri());
        let response_json: WhistInfo = service.get_info().await.unwrap();
        assert_eq!(response_json, expected_info);
    }
}