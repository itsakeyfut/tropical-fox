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
mod hot_asset;
mod plugins;
mod resources;
mod systems;

use combat::CombatPlugin;
use config::{
    BossesConfig, EnemiesConfig, GameSettings, PlayersConfig, SelectedCharacter,
    load_players_config_optional, load_settings_or_default,
};
use plugins::{AnimationPlugin, CorePlugin, EnemyPlugin, PlayerPlugin};

#[cfg(debug_assertions)]
use hot_asset::HotReloadPlugin;

#[cfg(debug_assertions)]
use systems::{
    apply_bosses_config_reload, apply_enemies_config_reload, apply_game_settings_reload,
    apply_players_config_reload,
};

fn main() {
    let settings = load_settings_or_default("assets/config/game_settings.ron");

    // Load player configuration and set selected character
    let selected_character =
        if let Some(players_config) = load_players_config_optional("assets/config/players.ron") {
            SelectedCharacter::new(players_config.default_player.clone())
        } else {
            SelectedCharacter::default()
        };

    let mut app = App::new();

    app.add_plugins(
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
            .set(ImagePlugin::default_nearest())
            // Enable asset hot-reload in debug builds
            .set({
                #[cfg(debug_assertions)]
                {
                    use bevy::asset::AssetPlugin;
                    AssetPlugin {
                        file_path: "assets".to_string(),
                        processed_file_path: ".imported_assets".to_string(),
                        watch_for_changes_override: Some(true),
                        ..default()
                    }
                }
                #[cfg(not(debug_assertions))]
                {
                    AssetPlugin::default()
                }
            }),
    )
    .insert_resource(settings)
    .insert_resource(selected_character)
    .add_plugins((
        CorePlugin,
        AnimationPlugin,
        PlayerPlugin,
        CombatPlugin,
        EnemyPlugin,
    ));

    #[cfg(debug_assertions)]
    {
        app.add_plugins(
            HotReloadPlugin::<GameSettings>::new("config/game_settings.ron")
                .with_verbose(true)
                .with_metrics(true),
        );

        app.add_plugins(
            HotReloadPlugin::<EnemiesConfig>::new("config/enemies.ron").with_verbose(true),
        );

        app.add_plugins(
            HotReloadPlugin::<PlayersConfig>::new("config/players.ron").with_verbose(true),
        );

        app.add_plugins(
            HotReloadPlugin::<BossesConfig>::new("config/bosses.ron").with_verbose(true),
        );

        app.add_systems(
            Update,
            (
                apply_game_settings_reload,
                apply_enemies_config_reload,
                apply_players_config_reload,
                apply_bosses_config_reload,
            ),
        );
    }

    app.run();
}
