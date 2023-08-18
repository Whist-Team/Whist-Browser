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

    #[cfg(not(target_family = "wasm"))]
    {
        use bevy::asset::ChangeWatcher;
        use std::time::Duration;

        println!("Enabling filesystem watcher for asset reload");
        plugins = plugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            ..default()
        });
    }

    App::new()
        .add_plugins(plugins)
        .add_plugins(GamePlugin)
        .run();
}
