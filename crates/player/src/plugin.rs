//! Player plugin
//!
//! Handles player entity spawning and related systems.

use bevy::prelude::*;
use tropical_fox_animation::{AnimationClip, AnimationController, AnimationState};
use tropical_fox_combat::{AttackCooldown, Health, Lives, PlayerHealth, PlayerSpawnPoint};
use tropical_fox_common::{
    CharacterAssets, CharacterTextureAtlas, Collider, GameState, Gravity, GroundDetection, Player,
    PlayerStats, Velocity,
};

use crate::config::SelectedCharacter;
use crate::systems::{
    flip_sprite_by_facing, ground_collision, player_dash, player_horizontal_movement, player_jump,
    update_dash, variable_jump_height, wall_collision, wall_jump, wall_slide,
};
use tropical_fox_animation::systems::player_animation_controller;

/// Plugin that manages the player entity and systems
pub struct PlayerPlugin;

/// Load player character assets from configuration
fn load_player_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut character_assets: ResMut<CharacterAssets>,
) {
    use tropical_fox_animation::load_animation_config_optional;

    info!("Loading player character assets");

    // Load players configuration
    let players_config =
        match crate::config::load_players_config_optional("assets/config/players.ron") {
            Some(config) => config,
            None => {
                warn!("Could not load players.ron, skipping player asset loading");
                return;
            }
        };

    // Load each player's assets
    for (player_id, player_def) in &players_config.players {
        let animation_config_path = format!("assets/{}", player_def.animation_config_path);

        // Try to load animation config
        let anim_config = match load_animation_config_optional(&animation_config_path) {
            Some(config) => config,
            None => {
                warn!(
                    "Could not load animation config for player '{}' at {}",
                    player_id, animation_config_path
                );
                continue;
            }
        };

        // Load sprite sheet texture
        let texture = asset_server.load(&anim_config.spritesheet_path);

        // Create texture atlas layout
        let sprite_size = UVec2::new(anim_config.sprite_size.0, anim_config.sprite_size.1);
        let layout = TextureAtlasLayout::from_grid(
            sprite_size,
            anim_config.columns,
            anim_config.rows,
            None,
            None,
        );
        let layout_handle = texture_atlas_layouts.add(layout);

        // Insert into character assets
        character_assets.insert(
            player_id,
            CharacterTextureAtlas::new(texture, layout_handle),
        );

        info!(
            "Loaded player character '{}': {}x{} sprites, {}x{} grid",
            player_id, sprite_size.x, sprite_size.y, anim_config.columns, anim_config.rows
        );
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add spawn point resource
        app.insert_resource(PlayerSpawnPoint::default());

        // Load player character assets in PostStartup
        // This ensures AnimationPlugin's Startup system has created CharacterAssets resource
        app.add_systems(PostStartup, load_player_assets);

        // Spawn player and test level when entering InGame state
        app.add_systems(OnEnter(GameState::InGame), (spawn_player, spawn_test_walls));

        // Player input and movement systems (run in Update)
        app.add_systems(
            Update,
            (
                player_horizontal_movement,
                player_jump,
                wall_jump,
                variable_jump_height,
                player_dash,
                update_dash,
                wall_slide,
            )
                .run_if(in_state(GameState::InGame)),
        );

        // Collision and sprite systems (run in FixedUpdate after physics)
        // Animation controller must run after ground_collision to use updated is_grounded
        app.add_systems(
            FixedUpdate,
            (
                ground_collision,
                wall_collision,
                flip_sprite_by_facing,
                player_animation_controller,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

/// Create fallback animation controller with hardcoded animations
///
/// Used when animation config file fails to load or parse
fn create_fallback_animation_controller() -> (AnimationController, AnimationState) {
    info!("Setting up fallback player animations");
    let mut controller = AnimationController::new();
    // These unwrap() calls are safe because they use hardcoded valid values
    controller.add_animation("idle", AnimationClip::new(0, 3, 4.0).unwrap());
    controller.add_animation("run", AnimationClip::new(6, 11, 12.0).unwrap());
    controller.add_animation("climb", AnimationClip::new(12, 15, 10.0).unwrap());
    controller.add_animation("crouch", AnimationClip::new(18, 20, 10.0).unwrap());
    controller.add_animation("hurt", AnimationClip::new(24, 25, 15.0).unwrap());
    controller.add_animation("jump", AnimationClip::new(30, 30, 1.0).unwrap());
    controller.add_animation("fall", AnimationClip::new(31, 31, 1.0).unwrap());
    controller.add_animation("dizzy", AnimationClip::new(48, 53, 8.0).unwrap());
    controller.add_animation("roll", AnimationClip::new(54, 57, 12.0).unwrap());
    controller.add_animation("look_up", AnimationClip::new(60, 60, 1.0).unwrap());
    controller.add_animation("win", AnimationClip::new(66, 66, 1.0).unwrap());
    controller.current_animation = "idle".to_string();
    controller.with_initial_state(true)
}
/// Spawn the player entity
fn spawn_player(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    character_assets: Option<Res<CharacterAssets>>,
    selected_character: Option<Res<SelectedCharacter>>,
) {
    use tropical_fox_animation::load_animation_config_optional;

    // Character display size (2x the sprite size for better visibility)
    let player_size = Vec2::new(64.0, 64.0);

    // Determine which character to use
    let character_id = selected_character
        .as_ref()
        .map(|sc| sc.character_id.as_str())
        .unwrap_or("fox");

    info!("Spawning player with character: {}", character_id);

    // Try to load animation config from file, fallback to hardcoded if failed
    let (animation_controller, animation_state) = {
        let players_config =
            crate::config::load_players_config_optional("assets/config/players.ron");

        if let Some(config) = players_config {
            if let Ok(player_def) = config.get_player(character_id) {
                let animation_config_path = format!("assets/{}", player_def.animation_config_path);

                if let Some(anim_config) = load_animation_config_optional(&animation_config_path) {
                    match AnimationController::try_from(anim_config) {
                        Ok(controller) => {
                            info!(
                                "Loaded player animations from {} ({} clips)",
                                animation_config_path,
                                controller.animations.len()
                            );
                            controller.with_initial_state(true)
                        }
                        Err(e) => {
                            warn!(
                                "Failed to create animation controller from {}: {}. Using fallback.",
                                animation_config_path, e
                            );
                            create_fallback_animation_controller()
                        }
                    }
                } else {
                    warn!(
                        "Could not load animation config at {}. Using fallback.",
                        animation_config_path
                    );
                    create_fallback_animation_controller()
                }
            } else {
                warn!(
                    "Player '{}' not found in config. Using fallback.",
                    character_id
                );
                create_fallback_animation_controller()
            }
        } else {
            warn!("Could not load players.ron. Using fallback animations.");
            create_fallback_animation_controller()
        }
    };

    // Check if we have character assets loaded
    if let Some(assets) = character_assets {
        // Get assets for the selected character, falling back to default if not found
        let character_atlas = assets.get(character_id).or_else(|| {
            warn!(
                "Assets for '{}' not found, using default (fox)",
                character_id
            );
            assets.get_default()
        });

        if let Some(atlas) = character_atlas {
            // Spawn with texture atlas
            let mut entity = commands.spawn(Transform::from_xyz(0.0, 100.0, 0.0));

            entity.insert(Visibility::default());
            entity.insert(Sprite {
                image: atlas.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: 0,
                }),
                custom_size: Some(player_size),
                ..default()
            });
            entity.insert(Name::new("Player"));

            entity.insert(Player::default());
            entity.insert(PlayerStats::default());
            entity.insert(GroundDetection::default());

            entity.insert(Velocity::zero());
            entity.insert(Gravity::default());
            entity.insert(Collider::new(player_size));

            entity.insert(animation_controller);
            entity.insert(animation_state);

            // Combat components
            entity.insert(Health::new(100.0));
            entity.insert(PlayerHealth);
            entity.insert(Lives::new(3));
            entity.insert(AttackCooldown::default());

            info!(
                "Player spawned at position (0, 100) with character '{}' texture atlas",
                character_id
            );
        } else {
            // Fallback to colored square if no assets are available
            warn!(
                "No character assets available for '{}', spawning with colored square",
                character_id
            );
            spawn_player_with_placeholder(
                &mut commands,
                player_size,
                animation_controller,
                animation_state,
            );
        }
    } else {
        // Fallback to colored square if assets aren't loaded yet
        warn!("Character assets not loaded yet, spawning player with colored square");
        spawn_player_with_placeholder(
            &mut commands,
            player_size,
            animation_controller,
            animation_state,
        );
    }
}

/// Spawn player with a colored placeholder square
fn spawn_player_with_placeholder(
    commands: &mut Commands,
    player_size: Vec2,
    animation_controller: AnimationController,
    animation_state: AnimationState,
) {
    let mut entity = commands.spawn(Transform::from_xyz(0.0, 100.0, 0.0));

    entity.insert(Visibility::default());
    entity.insert(Sprite {
        color: Color::srgb(0.2, 0.7, 0.3), // Green placeholder
        custom_size: Some(player_size),
        ..default()
    });
    entity.insert(Name::new("Player"));

    entity.insert(Player::default());
    entity.insert(PlayerStats::default());
    entity.insert(GroundDetection::default());

    entity.insert(Velocity::zero());
    entity.insert(Gravity::default());
    entity.insert(Collider::new(player_size));

    entity.insert(animation_controller);
    entity.insert(animation_state);

    // Combat components
    entity.insert(Health::new(100.0));
    entity.insert(PlayerHealth);
    entity.insert(Lives::new(3));
    entity.insert(AttackCooldown::default());

    info!("Player spawned at position (0, 100) with placeholder sprite");
}

/// Spawn a ground platform for testing
pub fn spawn_test_ground(mut commands: Commands) {
    let ground_size = Vec2::new(800.0, 32.0);

    commands.spawn((
        // Transform and visibility
        Transform::from_xyz(0.0, -200.0, 0.0),
        Visibility::default(),
        // Sprite rendering
        Sprite {
            color: Color::srgb(0.5, 0.3, 0.2), // Brown ground
            custom_size: Some(ground_size),
            ..default()
        },
        // Ground-specific components
        tropical_fox_common::Ground,
        Collider::new(ground_size),
        // Name for debugging
        Name::new("Ground Platform"),
    ));

    info!("Test ground platform spawned at position (0, -200)");
}

/// Spawn test walls for wall-jump practice
pub fn spawn_test_walls(mut commands: Commands) {
    // Left wall
    let wall_size = Vec2::new(32.0, 500.0);
    commands.spawn((
        Transform::from_xyz(-350.0, 50.0, 0.0),
        Visibility::default(),
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.5), // Gray wall
            custom_size: Some(wall_size),
            ..default()
        },
        tropical_fox_common::Wall,
        Collider::new(wall_size),
        Name::new("Left Wall"),
    ));

    // Right wall
    commands.spawn((
        Transform::from_xyz(350.0, 50.0, 0.0),
        Visibility::default(),
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.5), // Gray wall
            custom_size: Some(wall_size),
            ..default()
        },
        tropical_fox_common::Wall,
        Collider::new(wall_size),
        Name::new("Right Wall"),
    ));

    // Platform in the middle (optional - for easier testing)
    let platform_size = Vec2::new(150.0, 20.0);
    commands.spawn((
        Transform::from_xyz(0.0, -50.0, 0.0),
        Visibility::default(),
        Sprite {
            color: Color::srgb(0.3, 0.5, 0.3), // Green platform
            custom_size: Some(platform_size),
            ..default()
        },
        tropical_fox_common::Ground,
        Collider::new(platform_size),
        Name::new("Middle Platform"),
    ));

    info!("Test walls spawned at positions (-350, 50) and (350, 50)");
}
