use serde::{Deserialize, Serialize};

/// Inner layer containing a details.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameInfo {
    ///name of the current supported game
    game: String,
    ///version of the Whist-Core
    version: String,
}

/// Outer layer of the info object return from Whist-Server
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhistInfo {
    /// inner layer containing all server details
    info: GameInfo,
}

/// Implementation of the WhistInfo
impl WhistInfo {
    /// # Arguments
    ///
    /// * 'game' - The name of the game support
    /// * 'version' - The core version of the above game.
    pub fn new(game: impl Into<String>, version: impl Into<String>) -> WhistInfo {
        WhistInfo {
            info: GameInfo {
                game: game.into(),
                version: version.into(),
            },
        }
    }
}
