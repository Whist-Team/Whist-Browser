use bevy::ecs::event::*;
use reqwest::{Error, IntoUrl, Method};

use crate::network::*;

const BEARER_TOKEN_TYPE: &str = "Bearer";

#[derive(Debug)]
pub enum ConnectError {
    Request(Error),
    Requirement(RequirementError),
}

impl From<Error> for ConnectError {
    fn from(error: Error) -> Self {
        ConnectError::Request(error)
    }
}

impl From<RequirementError> for ConnectError {
    fn from(error: RequirementError) -> Self {
        ConnectError::Requirement(error)
    }
}

/// Service to provide call to whist server routes.
pub struct ServerService {
    server_connection: ServerConnection,
}

#[derive(Debug, Event)]
pub struct UserCreateResult(pub Result<UserCreateResponse, Error>);

#[derive(Debug, Event)]
pub struct GameListResult(pub Result<GameListResponse, Error>);

#[derive(Debug, Event)]
pub struct GameJoinResult(pub Result<GameJoinResponse, Error>);

#[derive(Debug, Event)]
pub struct GameReconnectResult(pub Result<GameReconnectResponse, Error>);

#[derive(Debug, Event)]
pub struct GameCreateResult(pub Result<GameCreateResponse, Error>);

impl ServerService {
    /// Constructor
    /// # Arguments
    /// * 'base_url' the url of the server
    pub fn new(base_url: impl IntoUrl) -> Self {
        Self {
            server_connection: ServerConnection::new(base_url),
        }
    }

    /// Retrieves the whist info object from the server.
    pub async fn get_info(&self) -> Result<WhistInfo, Error> {
        self.server_connection
            .request_with_json_result(Method::GET, "", Query::<()>::None, Body::<()>::Empty, None)
            .await
    }

    pub async fn check_connection(&self) -> Result<(), ConnectError> {
        self.get_info().await?.check_validity(&WhistInfoReq::new(
            crate::EXPECTED_GAME,
            crate::EXPECTED_CORE_VERSION,
            crate::EXPECTED_SERVER_VERSION,
        ))?;
        Ok(())
    }

    pub async fn login(&mut self, body: &LoginForm) -> Result<(), LoginError> {
        let res: LoginResponse = self
            .server_connection
            .request_with_json_result(
                Method::POST,
                "user/auth",
                Query::<()>::None,
                Body::Form(body),
                None,
            )
            .await?;
        if BEARER_TOKEN_TYPE == res.token_type {
            self.server_connection.token(res.access_token);
            Ok(())
        } else {
            Err(LoginError::UnknownTokenType(res.token_type))
        }
    }

    pub async fn github_auth(&mut self, body: &SwapTokenRequest) -> Result<(), LoginError> {
        let res: LoginResponse = self
            .server_connection
            .request_with_json_result(
                Method::POST,
                "oauth2/github/device",
                Query::<()>::None,
                Body::Json(body),
                None,
            )
            .await?;
        if BEARER_TOKEN_TYPE == res.token_type {
            self.server_connection.token(res.access_token);
            Ok(())
        } else {
            Err(LoginError::UnknownTokenType(res.token_type))
        }
    }

    pub async fn create_user(&self, body: &UserCreateRequest) -> UserCreateResult {
        UserCreateResult(
            self.server_connection
                .request_with_json_result(
                    Method::POST,
                    "user/create",
                    Query::<()>::None,
                    Body::Json(body),
                    None,
                )
                .await,
        )
    }

    pub async fn get_games(&self) -> GameListResult {
        GameListResult(
            self.server_connection
                .request_with_json_result(
                    Method::GET,
                    "room/info/ids",
                    Query::<()>::None,
                    Body::<()>::Empty,
                    None,
                )
                .await,
        )
    }

    pub async fn join_game(
        &self,
        game_id: impl AsRef<str>,
        body: &GameJoinRequest,
    ) -> GameJoinResult {
        GameJoinResult(
            self.server_connection
                .request_with_json_result(
                    Method::POST,
                    format!("room/join/{}", game_id.as_ref()),
                    Query::<()>::None,
                    Body::Json(body),
                    None,
                )
                .await,
        )
    }

    pub async fn reconnect(&self) -> GameReconnectResult {
        GameReconnectResult(
            self.server_connection
                .request_with_json_result(
                    Method::POST,
                    "room/reconnect/",
                    Query::<()>::None,
                    Body::<()>::Empty,
                    None,
                )
                .await,
        )
    }

    pub async fn create_game(&self, body: &GameCreateRequest) -> GameCreateResult {
        GameCreateResult(
            self.server_connection
                .request_with_json_result(
                    Method::POST,
                    "room/create",
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
        let expected_info = WhistInfo::new("whist", "0.1.0", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.to_owned()))
            .mount(&mock_server)
            .await;
        let service = ServerService::new(mock_server.uri());
        let response_json = service.get_info().await.unwrap();
        assert_eq!(response_json, expected_info);
    }
}
