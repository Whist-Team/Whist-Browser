use reqwest::Error;
use serde::{Deserialize, Serialize};

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
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> LoginForm {
        LoginForm {
            username: username.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
}

impl LoginResponse {
    pub fn new(token: impl Into<String>, token_type: impl Into<String>) -> LoginResponse {
        LoginResponse {
            token: token.into(),
            token_type: token_type.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub password: String,
}

impl UserCreateRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> UserCreateRequest {
        UserCreateRequest {
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
    pub fn new(user_id: impl Into<String>) -> UserCreateResponse {
        UserCreateResponse {
            user_id: user_id.into(),
        }
    }
}
