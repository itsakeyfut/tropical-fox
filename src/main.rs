//! Tropical Fox - A 2D action game
//!
//! This is the main entry point for the game.

use bevy::prelude::*;

mod components;
mod config;
mod debug;
mod events;
mod game_state;
mod plugins;
mod resources;
mod systems;

use plugins::CorePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tropical Fox".to_string(),
                        resolution: (1280, 720).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(CorePlugin)
        .run();
}
