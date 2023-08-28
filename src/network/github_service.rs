use bevy::prelude::Event;
use reqwest::{Error, IntoUrl, Method};

use crate::network::{Body, GitHubAuthRequest, GitHubTempTokenResponse, Query, ServerConnection};

/// Service to provide call to github routes.
pub struct GitHubService {
    server_connection: ServerConnection,
}

#[derive(Debug, Event)]
pub struct GitHubTempTokenResult(pub Result<GitHubTempTokenResponse, Error>);

impl GitHubService {
    /// Constructor
    /// # Arguments
    /// * 'base_url' the url of the server
    pub fn new(base_url: impl IntoUrl) -> Self {
        Self {
            server_connection: ServerConnection::new(base_url),
        }
    }

    pub async fn request_github_auth(&self, body: &GitHubAuthRequest) -> GitHubTempTokenResult {
        GitHubTempTokenResult(
            self.server_connection
                .request_with_json_result(
                    Method::POST,
                    "login/device/code",
                    Query::<()>::None,
                    Body::Json(body),
                    None,
                )
                .await,
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::network::*;

    #[tokio::test]
    async fn test_get_json() {
        let expected_info =
            GitHubTempTokenResponse::new("abc", 900, 5, "cde", "https://github.com/login/device");
        let auth_request = GitHubAuthRequest::new("abc");
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.to_owned()))
            .mount(&mock_server)
            .await;
        let service = GitHubService::new(mock_server.uri());
        let response_json = service.request_github_auth(&auth_request).await.0.unwrap();
        assert_eq!(response_json, expected_info);
    }
}
