//! Hot-reload systems for configuration updates
//!
//! These systems react to configuration file changes and update game state accordingly.

use crate::components::{Player, PlayerStats};
use crate::config::{BossesConfig, EnemiesConfig, GameSettings, PlayersConfig};
use crate::hot_asset::{AssetReloaded, HotAssetHandle};
use crate::resources::PhysicsConfig;
use bevy::prelude::*;

/// System that applies reloaded game settings to the game state
pub fn apply_game_settings_reload(
    mut events: MessageReader<AssetReloaded<GameSettings>>,
    handle: Res<HotAssetHandle<GameSettings>>,
    assets: Res<Assets<GameSettings>>,
    mut settings_resource: ResMut<GameSettings>,
    mut physics_config: ResMut<PhysicsConfig>,
    mut player_query: Query<&mut PlayerStats, With<Player>>,
) {
    for event in events.read() {
        if !event.success {
            continue;
        }

        if let Some(new_settings) = assets.get(&handle.0) {
            info!("⚙️ Applying reloaded game_settings.ron...");

            // Update physics config
            physics_config.gravity = new_settings.physics.gravity;
            physics_config.terminal_velocity = new_settings.physics.terminal_velocity;
            info!(
                "  Updated physics: gravity={}, terminal_velocity={}",
                new_settings.physics.gravity, new_settings.physics.terminal_velocity
            );

            // Update all player entities
            let mut player_count = 0;
            for mut stats in player_query.iter_mut() {
                stats.move_speed = new_settings.player.move_speed;
                stats.acceleration = new_settings.player.acceleration;
                stats.deceleration = new_settings.player.deceleration;
                stats.air_deceleration = new_settings.player.air_deceleration;
                stats.jump_force = new_settings.player.jump_force;
                stats.coyote_time = new_settings.player.coyote_time;
                stats.jump_buffer_time = new_settings.player.jump_buffer_time;
                stats.jump_cut_multiplier = new_settings.player.jump_cut_multiplier;
                stats.dash_speed = new_settings.player.dash_speed;
                stats.dash_duration = new_settings.player.dash_duration;
                stats.max_air_dashes = new_settings.player.max_air_dashes;
                stats.wall_jump_force_x = new_settings.player.wall_jump_force_x;
                stats.wall_jump_force_y = new_settings.player.wall_jump_force_y;
                stats.wall_slide_speed = new_settings.player.wall_slide_speed;
                player_count += 1;
            }

            if player_count > 0 {
                info!("  Updated {} player entities", player_count);
            }

            // Update the GameSettings resource
            *settings_resource = new_settings.clone();

            info!("✅ Successfully applied reloaded game settings");
        }
    }
}

/// System that applies reloaded enemy configuration to the game state
pub fn apply_enemies_config_reload(
    mut events: MessageReader<AssetReloaded<EnemiesConfig>>,
    handle: Res<HotAssetHandle<EnemiesConfig>>,
    assets: Res<Assets<EnemiesConfig>>,
    mut enemy_query: Query<(
        &crate::components::Enemy,
        &mut crate::components::EnemyStats,
        &mut crate::components::EnemyAI,
        Option<&mut crate::components::ProjectileShooter>,
    )>,
) {
    for event in events.read() {
        if !event.success {
            continue;
        }

        if let Some(new_config) = assets.get(&handle.0) {
            info!("👾 Applying reloaded enemies.ron...");

            let mut updated_count = 0;

            for (enemy, mut stats, mut ai, shooter) in enemy_query.iter_mut() {
                if let Some(config) = new_config.enemies.get(&enemy.enemy_type) {
                    // Update stats
                    stats.move_speed = config.stats.move_speed;
                    stats.damage = config.stats.damage;
                    stats.knockback_force = config.stats.knockback_force;
                    stats.score_value = config.stats.score_value;

                    // Update AI while preserving runtime state
                    update_ai_preserving_state(&mut ai, &config.ai);

                    // Update projectile shooter if present
                    if config.projectile.enabled {
                        if let Some(mut shooter) = shooter {
                            shooter.damage = config.projectile.damage;
                            shooter.projectile_speed = config.projectile.speed;
                            shooter.fire_rate = config.projectile.fire_rate;
                            shooter.range = config.projectile.range;
                        }
                    }

                    updated_count += 1;
                }
            }

            if updated_count > 0 {
                info!("  Updated {} enemy entities", updated_count);
            }

            info!("✅ Successfully applied reloaded enemies config");
        }
    }
}

/// Updates AI configuration while preserving runtime state
fn update_ai_preserving_state(
    current_ai: &mut crate::components::EnemyAI,
    new_config: &crate::config::AIConfig,
) {
    use crate::components::EnemyAI;
    use crate::config::AIBehaviorType;

    match current_ai {
        EnemyAI::Patrol(patrol) => {
            if matches!(new_config.behavior, AIBehaviorType::Patrol) {
                // Update wait_time (preserve other runtime state)
                patrol.wait_time = new_config.wait_time;
                info!("  Updated Patrol AI: wait_time={}", patrol.wait_time);
            } else {
                // AI type changed - would need to respawn for full change
                warn!(
                    "  AI behavior changed from Patrol to {:?} - full update requires respawn",
                    new_config.behavior
                );
            }
        }
        EnemyAI::Chase(chase) => {
            if matches!(new_config.behavior, AIBehaviorType::Chase) {
                // Update config values (preserve runtime state like is_chasing, lost_sight_timer)
                chase.detection_range = new_config.detection_range;
                chase.attack_range = new_config.attack_range;

                info!(
                    "  Updated Chase AI: detection={}, attack_range={}",
                    chase.detection_range, chase.attack_range
                );
            } else {
                warn!(
                    "  AI behavior changed from Chase to {:?} - full update requires respawn",
                    new_config.behavior
                );
            }
        }
        EnemyAI::Flying(_flying) => {
            if matches!(new_config.behavior, AIBehaviorType::Flying) {
                // Flying AI pattern is complex and set at spawn time
                info!("  Flying AI detected - pattern updates require respawn");
            } else {
                warn!(
                    "  AI behavior changed from Flying to {:?} - full update requires respawn",
                    new_config.behavior
                );
            }
        }
        EnemyAI::Idle => {
            if !matches!(new_config.behavior, AIBehaviorType::Idle) {
                warn!(
                    "  AI behavior changed from Idle to {:?} - full update requires respawn",
                    new_config.behavior
                );
            }
        }
    }
}

/// System that applies reloaded players configuration to the game state
pub fn apply_players_config_reload(
    mut events: MessageReader<AssetReloaded<PlayersConfig>>,
    handle: Res<HotAssetHandle<PlayersConfig>>,
    assets: Res<Assets<PlayersConfig>>,
) {
    for event in events.read() {
        if !event.success {
            continue;
        }

        if let Some(new_config) = assets.get(&handle.0) {
            info!("🦊 Applying reloaded players.ron...");
            info!("  Total player types: {}", new_config.players.len());
            info!("  Default player: {}", new_config.default_player);

            // Note: Player configuration is typically used at spawn time
            // Existing player entities won't be automatically updated
            // To apply changes, respawn the player entity

            info!("✅ Successfully applied reloaded players config");
        }
    }
}

/// System that applies reloaded bosses configuration to the game state
pub fn apply_bosses_config_reload(
    mut events: MessageReader<AssetReloaded<BossesConfig>>,
    handle: Res<HotAssetHandle<BossesConfig>>,
    assets: Res<Assets<BossesConfig>>,
) {
    for event in events.read() {
        if !event.success {
            continue;
        }

        if let Some(new_config) = assets.get(&handle.0) {
            info!("🐉 Applying reloaded bosses.ron...");
            info!("  Total boss types: {}", new_config.bosses.len());

            // Note: Boss configuration is typically used at spawn time
            // Existing boss entities won't be automatically updated
            // To apply changes, respawn the boss entity

            info!("✅ Successfully applied reloaded bosses config");
        }
    }
}
