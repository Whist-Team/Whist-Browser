use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConnectError {
    Request(Error),
    GameInvalid(String),
    VersionInvalid(String),
}

impl From<Error> for ConnectError {
    fn from(error: Error) -> Self {
        ConnectError::Request(error)
    }
}

/// Inner layer containing the details.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameInfo {
    ///name of the current supported game
    pub game: String,
    ///version of the Whist-Core
    pub version: String,
}

/// Outer layer of the info object return from Whist-Server
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhistInfo {
    /// inner layer containing all server details
    pub info: GameInfo,
}

/// Implementation of the WhistInfo
impl WhistInfo {
    /// # Arguments
    ///
    /// * 'game' - The name of the game support
    /// * 'version' - The core version of the above game.
    pub fn new(game: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            info: GameInfo {
                game: game.into(),
                version: version.into(),
            },
        }
    }
}
