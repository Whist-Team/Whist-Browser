use reqwest::{Error, IntoUrl, Method};

use crate::network::*;

const BEARER_TOKEN_TYPE: &str = "Bearer";

/// Service to provide call to whist server routes.
pub struct ServerService {
    server_connection: ServerConnection,
}

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
            .request_with_json_result(Method::GET, "", Query::<()>::None, Body::<()>::Empty)
            .await
    }

    pub async fn check_connection(&self) -> Result<(), ConnectError> {
        let info = self.get_info().await?.info;
        if info.game != crate::EXPECTED_GAME {
            Err(ConnectError::GameInvalid(info.game))
        } else if info.version != crate::EXPECTED_VERSION {
            Err(ConnectError::VersionInvalid(info.version))
        } else {
            Ok(())
        }
    }

    pub async fn login(&mut self, body: &LoginForm) -> Result<(), LoginError> {
        let res: LoginResponse = self
            .server_connection
            .request_with_json_result(
                Method::POST,
                "user/auth",
                Query::<()>::None,
                Body::Form(body),
            )
            .await?;
        if BEARER_TOKEN_TYPE == res.token_type {
            self.server_connection.token(res.token);
            Ok(())
        } else {
            Err(LoginError::UnknownTokenType(res.token_type))
        }
    }

    pub async fn create_user(&self, body: &UserCreateRequest) -> Result<UserCreateResponse, Error> {
        self.server_connection
            .request_with_json_result(
                Method::POST,
                "user/auth/create",
                Query::<()>::None,
                Body::Json(body),
            )
            .await
    }

    pub async fn get_games(&self) -> Result<GameListResponse, Error> {
        self.server_connection
            .request_with_json_result(
                Method::GET,
                "game/info/ids",
                Query::<()>::None,
                Body::<()>::Empty,
            )
            .await
    }

    pub async fn join_game(
        &self,
        game_id: impl AsRef<str>,
        body: &GameJoinRequest,
    ) -> Result<GameJoinResponse, Error> {
        self.server_connection
            .request_with_json_result(
                Method::GET,
                format!("game/join/{}", game_id.as_ref()),
                Query::<()>::None,
                Body::Json(body),
            )
            .await
    }

    pub async fn create_game(&self, body: &GameCreateRequest) -> Result<GameCreateResponse, Error> {
        self.server_connection
            .request_with_json_result(
                Method::GET,
                "game/create",
                Query::<()>::None,
                Body::Json(body),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::network::*;

    #[tokio::test]
    async fn test_get_json() {
        let expected_info = WhistInfo::new("whist", "0.1.0");

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
