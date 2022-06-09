use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Inner layer containing a details.
struct GameInfo {
    ///name of the current supported game
    game: String,
    ///version of the Whist-Core
    version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Outer layer of the info object return from Whist-Server
pub struct WhistInfo {
    /// inner layer containing all server details
    info: GameInfo,
}

/// Factory for the Whist Info structure
pub struct WhistInfoFactory;

/// Implementation of the factory
/// # Arguments
///
/// * 'game' - The name of the game support
/// * 'version' - The core version of the above game.
impl WhistInfoFactory {
    pub fn new_info(game: String, version: String) -> WhistInfo {
        WhistInfo {
            info: GameInfo {
                game,
                version,
            },
        }
    }
}
