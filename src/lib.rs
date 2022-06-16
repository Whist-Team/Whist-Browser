use bevy::prelude::*;

use crate::assets::LoadingPlugin;
use crate::connect::ConnectMenuPlugin;
use crate::ui::BaseUiPlugin;

mod assets;
mod card;
mod connect;
mod network;
mod ui;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingAssets,
    ConnectMenu,
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
            .add_plugin(ConnectMenuPlugin);
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
