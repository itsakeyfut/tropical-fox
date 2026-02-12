//! Tropical Fox - A 2D action game
//!
//! This is the main entry point for the game.

use bevy::prelude::*;

mod config;
mod core_plugin;
mod debug;
mod physics_systems;

// Import from workspace crates
use tropical_fox_animation::AnimationPlugin;
use tropical_fox_combat::CombatPlugin;
use tropical_fox_enemy::EnemyPlugin;
use tropical_fox_level::LevelPlugin;
use tropical_fox_player::{PlayerPlugin, SelectedCharacter};

use config::load_settings_or_default;
use core_plugin::CorePlugin;

#[cfg(debug_assertions)]
use tropical_fox_hot_asset::HotReloadPlugin;

#[cfg(debug_assertions)]
mod hot_reload_systems;
#[cfg(debug_assertions)]
use config::GameSettings;
#[cfg(debug_assertions)]
use hot_reload_systems::{
    apply_bosses_config_reload, apply_enemies_config_reload, apply_game_settings_reload,
    apply_players_config_reload,
};
#[cfg(debug_assertions)]
use tropical_fox_enemy::{BossesConfig, EnemiesConfig};
#[cfg(debug_assertions)]
use tropical_fox_level::LevelMetadataConfig;
#[cfg(debug_assertions)]
use tropical_fox_player::PlayersConfig;

fn main() {
    let settings = load_settings_or_default("assets/config/game_settings.ron");

    // Load player configuration and set selected character
    let selected_character =
        tropical_fox_player::load_players_config_optional("assets/config/players.ron")
            .map(|players_config| SelectedCharacter::new(players_config.default_player.clone()))
            .unwrap_or_default();

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
                use bevy::asset::AssetPlugin;

                // Find assets directory
                // Use absolute path to avoid Bevy resolving it relative to binary crate location
                let current_dir = std::env::current_dir().expect("Failed to get current directory");

                // Try multiple possible paths in order and convert to absolute path
                let possible_paths = ["assets", "../../assets", "../assets"];
                let asset_path = possible_paths
                    .iter()
                    .find_map(|&path| {
                        let full_path = current_dir.join(path);
                        if full_path.exists() {
                            full_path
                                .canonicalize()
                                .ok()
                                .map(|canonical| canonical.to_string_lossy().to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        eprintln!(
                            "Warning: Could not find assets directory from {:?}, using fallback",
                            current_dir
                        );
                        "assets".to_string()
                    });

                #[cfg(debug_assertions)]
                {
                    AssetPlugin {
                        file_path: asset_path,
                        processed_file_path: ".imported_assets".to_string(),
                        watch_for_changes_override: Some(true),
                        ..default()
                    }
                }
                #[cfg(not(debug_assertions))]
                {
                    AssetPlugin {
                        file_path: asset_path,
                        ..default()
                    }
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
        LevelPlugin,
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

        app.add_plugins(
            HotReloadPlugin::<LevelMetadataConfig>::new("config/levels_metadata.ron")
                .with_verbose(true),
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
