use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameListResponse {
    pub games: Vec<String>,
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
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameCreateResponse {
    pub game_id: String,
}
