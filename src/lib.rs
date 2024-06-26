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
mod network;
mod rooms;
mod ui;

pub const EXPECTED_GAME: &str = "whist";
pub const EXPECTED_CORE_VERSION: &str = "^0.9";
pub const EXPECTED_SERVER_VERSION: &str = "^0.7";

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
        app.init_state::<GameState>()
            // .configure_set(MySystemSets::EguiTop.after(CoreSet::Update))
            .add_plugins((
                BaseUiPlugin,
                LoadingPlugin,
                NetworkPlugin,
                ConnectMenuPlugin,
                LoginMenuPlugin,
                RoomMenuPlugin,
            ));
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
