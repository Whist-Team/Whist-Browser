// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use whist_browser::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Whist".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
