// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use whist_browser::GamePlugin;

#[bevy_main]
fn main() {
    let mut plugins = DefaultPlugins.build();
    plugins = plugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Whist".to_string(),
            width: 800.0,
            height: 600.0,
            ..default()
        },
        ..default()
    });

    #[cfg(not(target_family = "wasm"))]
    {
        println!("Enabling filesystem watcher for asset reload");
        plugins = plugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        });
    }

    App::new().add_plugins(plugins).add_plugin(GamePlugin).run();
}
