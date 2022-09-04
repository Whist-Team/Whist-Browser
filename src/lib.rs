use bevy::prelude::*;

use crate::assets::LoadingPlugin;
use crate::connect::ConnectMenuPlugin;
use crate::login::LoginMenuPlugin;
use crate::network::NetworkPlugin;
use crate::room::RoomLobbyPlugin;
use crate::rooms::RoomMenuPlugin;
use crate::ui::BaseUiPlugin;

mod assets;
mod card;
mod connect;
mod login;
mod network;
mod room;
mod rooms;
mod ui;

pub const EXPECTED_GAME: &str = "whist";
pub const EXPECTED_CORE_VERSION: &str = "^0.4";
pub const EXPECTED_SERVER_VERSION: &str = "^0.5";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingAssets,
    ConnectMenu,
    LoginMenu,
    RoomMenu,
    RoomLobby,
    Ingame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum MySystemLabel {
    EguiTop,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::LoadingAssets)
            .add_plugin(BaseUiPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(NetworkPlugin)
            .add_plugin(ConnectMenuPlugin)
            .add_plugin(LoginMenuPlugin)
            .add_plugin(RoomMenuPlugin)
            .add_plugin(RoomLobbyPlugin);
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
