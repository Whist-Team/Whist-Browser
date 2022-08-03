use crate::network::{Body, GitHubAuthRequest, GitHubTempTokenResponse, Query, ServerConnection};
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
        self.server_connection
            .request_with_json_result(
                Method::POST,
                "login/device/code",
                Query::<()>::None,
                Body::Json(body),
            )
            .await
    }
}
