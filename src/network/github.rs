pub struct GitHubAuthRequest {
    pub code: String,
}

pub struct GitHubTempTokenResponse {
    pub device_code: String,
    pub expires_in: u8,
    pub interval: u8,
    pub user_code: String,
    pub verification_uri: String,
}
