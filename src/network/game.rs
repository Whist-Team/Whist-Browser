use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameListResponse {
    pub rooms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameCreateRequest {
    pub game_name: String,
    pub password: Option<String>,
    pub min_player: Option<u8>,
    pub max_player: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameJoinRequest {
    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameJoinStatus {
    #[serde(rename = "joined")]
    Joined,
    #[serde(rename = "already joined")]
    AlreadyJoined,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameJoinResponse {
    pub status: GameJoinStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameCreateResponse {
    pub game_id: String,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::network::{GameCreateRequest, GameJoinResponse, GameJoinStatus};

    #[test]
    fn test_game_join_response_deserialize_1() {
        let expected = GameJoinResponse {
            status: GameJoinStatus::Joined,
        };
        let actual: GameJoinResponse = serde_json::from_value(json!({"status": "joined"})).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_game_join_response_deserialize_2() {
        let expected = GameJoinResponse {
            status: GameJoinStatus::AlreadyJoined,
        };
        let actual: GameJoinResponse =
            serde_json::from_value(json!({"status": "already joined"})).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_game_create_request_serialize_1() {
        let expected = json!({
            "game_name": "asdf",
            "password": null,
            "min_player": null,
            "max_player": null,
        });
        let actual = serde_json::to_value(&GameCreateRequest {
            game_name: "asdf".to_string(),
            password: None,
            min_player: None,
            max_player: None,
        })
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_game_create_request_serialize_2() {
        let expected = json!({
            "game_name": "asdf",
            "password": "12345_is_a_bad_password",
            "min_player": null,
            "max_player": null,
        });
        let actual = serde_json::to_value(&GameCreateRequest {
            game_name: "asdf".to_string(),
            password: Some("12345_is_a_bad_password".to_string()),
            min_player: None,
            max_player: None,
        })
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_game_create_request_serialize_3() {
        let expected = json!({
            "game_name": "asdf",
            "password": "12345_is_a_bad_password",
            "min_player": 4,
            "max_player": 4,
        });
        let actual = serde_json::to_value(&GameCreateRequest {
            game_name: "asdf".to_string(),
            password: Some("12345_is_a_bad_password".to_string()),
            min_player: Some(4),
            max_player: Some(4),
        })
        .unwrap();
        assert_eq!(expected, actual);
    }
}
