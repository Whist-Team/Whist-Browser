use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl fmt::Debug for GitHubAuthRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GitHubAuthRequest")
            .field("client_id", &"****")
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubTempTokenResponse {
    pub device_code: String,
    pub expires_in: i32,
    pub interval: u8,
    pub user_code: String,
    pub verification_uri: String,
}

impl GitHubTempTokenResponse {
    pub fn new(
        device_code: impl Into<String>,
        expires_in: impl Into<i32>,
        interval: impl Into<u8>,
        user_code: impl Into<String>,
        verification_uri: impl Into<String>,
    ) -> Self {
        Self {
            device_code: device_code.into(),
            expires_in: expires_in.into(),
            interval: interval.into(),
            user_code: user_code.into(),
            verification_uri: verification_uri.into(),
        }
    }
}

impl fmt::Debug for GitHubTempTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let interval = self.interval.to_string();
        let veri_uri = self.verification_uri.to_string();
        f.debug_struct("GitHubTempTokenResponse")
            .field("device_code", &"****")
            .field("interval", &interval)
            .field("user_code", &"****")
            .field("verification_uri", &veri_uri)
            .finish()
    }
}
