use reqwest::Error;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectError {
    Request(String),
    GameInvalid(String),
    CoreVersionInvalid(Version),
    ServerVersionInvalid(Version),
}

impl From<Error> for ConnectError {
    fn from(error: Error) -> Self {
        ConnectError::Request(error.to_string())
    }
}

/// Required properties of whist server and core, used for checking validity of server when connecting
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhistInfoReq {
    pub game: String,
    pub whist_core: VersionReq,
    pub whist_server: VersionReq,
}

impl WhistInfoReq {
    pub fn new(
        game: impl Into<String>,
        whist_core: impl AsRef<str>,
        whist_server: impl AsRef<str>,
    ) -> Self {
        Self {
            game: game.into(),
            whist_core: VersionReq::parse(whist_core.as_ref()).unwrap(),
            whist_server: VersionReq::parse(whist_server.as_ref()).unwrap(),
        }
    }
}

/// Inner layer containing the details.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GameInfo {
    ///name of the current supported game
    pub game: String,
    ///version of the Whist-Core
    pub whist_core: Version,
    ///version of the Whist-Server
    pub whist_server: Version,
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
        whist_core: impl AsRef<str>,
        whist_server: impl AsRef<str>,
    ) -> Self {
        Self {
            info: GameInfo {
                game: game.into(),
                whist_core: Version::parse(whist_core.as_ref()).unwrap(),
                whist_server: Version::parse(whist_server.as_ref()).unwrap(),
            },
        }
    }

    pub fn check_validity(&self, req: &WhistInfoReq) -> Result<(), ConnectError> {
        let info = &self.info;
        if !req.game.eq_ignore_ascii_case(&info.game) {
            Err(ConnectError::GameInvalid(info.game.to_owned()))
        } else if !req.whist_core.matches(&info.whist_core) {
            Err(ConnectError::CoreVersionInvalid(info.whist_core.to_owned()))
        } else if !req.whist_server.matches(&info.whist_server) {
            Err(ConnectError::ServerVersionInvalid(
                info.whist_server.to_owned(),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validity() {
        let req = WhistInfoReq::new("whist", "^0.2", "^0.1");
        let info = WhistInfo::new("WHIST", "0.2.0", "0.1.1");
        assert_eq!(info.check_validity(&req), Ok(()))
    }
}
