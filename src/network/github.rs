use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubAuthRequest {
    pub client_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubTempTokenResponse {
    pub device_code: String,
    pub expires_in: u8,
    pub interval: u8,
    pub user_code: String,
    pub verification_uri: String,
}
