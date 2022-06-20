// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use whist_browser::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Whist".to_string(),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
