use crate::network::{Body, GitHubAuthRequest, GitHubTempTokenResponse, Query, ServerConnection};
use reqwest::header::HeaderMap;
use reqwest::{Error, IntoUrl, Method};

/// Service to provide call to github routes.
pub struct GitHubService {
    server_connection: ServerConnection,
}

pub type GitHubTempTokenResult = Result<GitHubTempTokenResponse, Error>;

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
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        self.server_connection
            .request_with_json_result(
                Method::POST,
                "login/device/code",
                Query::<()>::None,
                Body::Json(body),
                Some(headers),
            )
            .await
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
        let response_json = service.request_github_auth(&auth_request).await.unwrap();
        assert_eq!(response_json, expected_info);
    }
}
