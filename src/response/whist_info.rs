use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GameInfo {
    game: String,
    version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhistInfo {
    info: GameInfo,

}
pub struct WhistInfoFactory;

impl WhistInfoFactory {
    pub fn new_info(game: &str, version: &str) -> WhistInfo{
        return WhistInfo {
            info: GameInfo {
                game: game.to_owned(),
                version: version.to_owned()
            }
        };
    }
}