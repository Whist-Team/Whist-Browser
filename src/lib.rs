use bevy::prelude::*;

use crate::assets::LoadingPlugin;
use crate::connect::ConnectMenuPlugin;
use crate::login::LoginMenuPlugin;
use crate::network::NetworkPlugin;
use crate::rooms::RoomMenuPlugin;
use crate::ui::BaseUiPlugin;

mod assets;
mod card;
mod connect;
mod login;
pub mod network;
mod rooms;
mod ui;

pub const EXPECTED_GAME: &str = "whist";
pub const EXPECTED_SERVER_VERSION: &str = "^0.5";

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingAssets,
    ConnectMenu,
    LoginMenu,
    RoomMenu,
    Ingame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum MySystemSets {
    EguiTop,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            // .configure_set(MySystemSets::EguiTop.after(CoreSet::Update))
            .add_plugin(BaseUiPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(NetworkPlugin)
            .add_plugin(ConnectMenuPlugin)
            .add_plugin(LoginMenuPlugin)
            .add_plugin(RoomMenuPlugin);
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
