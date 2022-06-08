use serde::{Serialize, Deserialize};

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
    pub fn new_info(game: &str, version: &str) -> WhistInfo {
        return WhistInfo {
            info: GameInfo {
                game: game.to_owned(),
                version: version.to_owned(),
            }
        };
    }
}