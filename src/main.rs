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

use config::load_settings_or_default;
use plugins::{CorePlugin, PlayerPlugin};

fn main() {
    let settings = load_settings_or_default("assets/config/game_settings.ron");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: settings.window.title.clone(),
                        resolution: (settings.window.width, settings.window.height).into(),
                        resizable: settings.window.resizable,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(settings)
        .add_plugins((CorePlugin, PlayerPlugin))
        .run();
}
