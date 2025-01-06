use crate::assets::LoadingPlugin;
use crate::connect::ConnectMenuPlugin;
use crate::login::LoginMenuPlugin;
use crate::network::NetworkPlugin;
use crate::rooms::RoomMenuPlugin;
use crate::ui::BaseUiPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiStartupSet;

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
    Starting,
    LoadingAssets,
    ConnectMenu,
    LoginMenu,
    RoomMenu,
    Ingame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum MySystemSets {
    Egui,
    EguiTop,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(
                PreStartup,
                MySystemSets::Egui.after(EguiStartupSet::InitContexts),
            )
            .configure_sets(
                Startup,
                MySystemSets::Egui.after(EguiStartupSet::InitContexts),
            )
            .configure_sets(
                PostStartup,
                MySystemSets::Egui.after(EguiStartupSet::InitContexts),
            )
            .configure_sets(Update, MySystemSets::EguiTop.run_if(egui_available))
            .configure_sets(
                Update,
                MySystemSets::Egui
                    .run_if(egui_available)
                    .after(MySystemSets::EguiTop),
            )
            .add_systems(PreStartup, switch_state.after(EguiStartupSet::InitContexts))
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

fn egui_available(query: Query<&PrimaryWindow>) -> bool {
    !query.is_empty()
}

fn switch_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::LoadingAssets);
}

pub fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
