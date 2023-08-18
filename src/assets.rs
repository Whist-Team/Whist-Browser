use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;

use crate::ui::{MONOSPACE_FONT, PROPORTIONAL_FONT};
use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingAssets), load_assets)
            .add_systems(
                Update,
                update_assets.run_if(in_state(GameState::LoadingAssets)),
            );
    }
}

#[derive(Resource)]
pub struct GameAssets {
    font: Handle<Font>,
    monospace_font: Handle<Font>,
}

#[allow(dead_code)]
impl GameAssets {
    fn new(font: impl Into<Handle<Font>>, monospace_font: impl Into<Handle<Font>>) -> Self {
        Self {
            font: font.into(),
            monospace_font: monospace_font.into(),
        }
    }

    fn get_handles(&self) -> impl IntoIterator<Item = HandleId> {
        []
    }

    pub fn font(&self) -> Handle<Font> {
        self.font.to_owned()
    }

    pub fn monospace_font(&self) -> Handle<Font> {
        self.monospace_font.to_owned()
    }
}

fn load_assets(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    /*server: Res<AssetServer>,*/
) {
    info!("loading assets...");
    commands.insert_resource(GameAssets::new(
        fonts.add(Font::try_from_bytes(Vec::from(PROPORTIONAL_FONT)).unwrap()),
        fonts.add(Font::try_from_bytes(Vec::from(MONOSPACE_FONT)).unwrap()),
    ));
}

fn update_assets(
    mut state: ResMut<NextState<GameState>>,
    server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    match server.get_group_load_state(assets.get_handles()) {
        LoadState::Failed | LoadState::Unloaded | LoadState::NotLoaded => {
            panic!("error loading assets")
        }
        LoadState::Loaded => state.set(GameState::ConnectMenu),
        _ => {}
    };
}
