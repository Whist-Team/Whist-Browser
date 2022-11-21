// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use whist_browser::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Whist".to_string(),
                width: 800.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
