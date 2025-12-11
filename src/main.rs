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

use config::{SelectedCharacter, load_characters_config_optional, load_settings_or_default};
use plugins::{AnimationPlugin, CorePlugin, PlayerPlugin};

fn main() {
    let settings = load_settings_or_default("assets/config/game_settings.ron");

    // Load character configuration and set selected character
    let selected_character = if let Some(characters_config) =
        load_characters_config_optional("assets/config/characters.ron")
    {
        SelectedCharacter::new(characters_config.default_player.clone())
    } else {
        SelectedCharacter::default()
    };

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
        .insert_resource(selected_character)
        .add_plugins((CorePlugin, AnimationPlugin, PlayerPlugin))
        .run();
}
