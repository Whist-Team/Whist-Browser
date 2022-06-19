use bevy::prelude::*;

use crate::assets::LoadingPlugin;
use crate::connect::ConnectMenuPlugin;
use crate::login::LoginMenuPlugin;
use crate::network::NetworkPlugin;
use crate::ui::BaseUiPlugin;

mod assets;
mod card;
mod connect;
mod login;
mod network;
mod ui;

pub const EXPECTED_GAME: &str = "whist";
pub const EXPECTED_VERSION: &str = "0.1.1";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingAssets,
    ConnectMenu,
    LoginMenu,
    RoomMenu,
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
            .add_plugin(LoginMenuPlugin);
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
