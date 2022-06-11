use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Inner layer containing a details.
pub struct GameInfo {
    ///name of the current supported game
    game: String,
    ///version of the Whist-Core
    version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Outer layer of the info object return from Whist-Server
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
