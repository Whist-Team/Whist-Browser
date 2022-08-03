use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum LoginError {
    Request(Error),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubAuthRequest {
    pub client_id: String,
}

impl GitHubAuthRequest {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubTempTokenResponse {
    pub device_code: String,
    pub expires_in: u8,
    pub interval: u8,
    pub user_code: String,
    pub verification_uri: String,
}