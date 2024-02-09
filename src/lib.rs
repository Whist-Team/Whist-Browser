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
mod player;
mod room;
mod rooms;
mod ui;

pub const EXPECTED_GAME: &str = "whist";
pub const EXPECTED_CORE_VERSION: &str = "^0.9";
pub const EXPECTED_SERVER_VERSION: &str = "^0.7";

#[derive(Default, Resource)]
struct Globals {
    room_id: Option<String>,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingAssets,
    ConnectMenu,
    LoginMenu,
    RoomMenu,
    RoomLobby,
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
            .add_plugins((
                BaseUiPlugin,
                LoadingPlugin,
                NetworkPlugin,
                ConnectMenuPlugin,
                LoginMenuPlugin,
                RoomMenuPlugin,
                RoomLobbyPlugin,
            ));
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
