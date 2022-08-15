use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum LoginError {
    Request(Error),
    UnknownTokenType(String),
}

impl From<Error> for LoginError {
    fn from(error: Error) -> Self {
        Self::Request(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

impl LoginForm {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

impl LoginResponse {
    pub fn new(access_token: impl Into<String>, token_type: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            token_type: token_type.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SwapTokenRequest {
    pub device_code: String,
}

impl SwapTokenRequest {
    pub fn new(device_code: impl Into<String>) -> Self {
        Self {
            device_code: device_code.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub password: String,
}

impl UserCreateRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserCreateResponse {
    pub user_id: String,
}

impl UserCreateResponse {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}
