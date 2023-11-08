// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowResolution;

use whist_browser::GamePlugin;

#[bevy_main]
fn main() {
    let mut plugins = DefaultPlugins.build();
    plugins = plugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Whist".to_string(),
            resolution: WindowResolution::new(800.0, 600.0),
            ..default()
        }),
        ..default()
    });

    App::new()
        .add_plugins(plugins)
        .add_plugins(GamePlugin)
        .run();
}
