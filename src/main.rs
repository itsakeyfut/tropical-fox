//! Tropical Fox - A 2D action game
//!
//! This is the main entry point for the game.

use bevy::prelude::*;

mod combat;
mod components;
mod config;
mod debug;
mod events;
mod game_state;
mod plugins;
mod resources;
mod systems;

use combat::CombatPlugin;
use config::{SelectedCharacter, load_players_config_optional, load_settings_or_default};
use plugins::{AnimationPlugin, CorePlugin, EnemyPlugin, LevelPlugin, PlayerPlugin};

fn main() {
    let settings = load_settings_or_default("assets/config/game_settings.ron");

    // Load player configuration and set selected character
    let selected_character =
        if let Some(players_config) = load_players_config_optional("assets/config/players.ron") {
            SelectedCharacter::new(players_config.default_player.clone())
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
        .add_plugins((
            CorePlugin,
            LevelPlugin,
            AnimationPlugin,
            PlayerPlugin,
            CombatPlugin,
            EnemyPlugin,
        ))
        .run();
}
