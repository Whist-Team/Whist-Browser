use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConnectError {
    Request(Error),
    GameInvalid(String),
    CoreVersionInvalid(String),
    ServerVersionInvalid(String),
}

impl From<Error> for ConnectError {
    fn from(error: Error) -> Self {
        ConnectError::Request(error)
    }
}

/// Inner layer containing the details.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GameInfo {
    ///name of the current supported game
    pub game: String,
    ///version of the Whist-Core
    pub whist_core: String,
    ///version of the Whist-Server
    pub whist_server: String,
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
    /// * 'whist_core' - The core version of the above game.
    /// * 'whist_server' - The server version of the above game.
    pub fn new(
        game: impl Into<String>,
        whist_core: impl Into<String>,
        whist_server: impl Into<String>,
    ) -> Self {
        Self {
            info: GameInfo {
                game: game.into(),
                whist_core: whist_core.into(),
                whist_server: whist_server.into(),
            },
        }
    }
}
