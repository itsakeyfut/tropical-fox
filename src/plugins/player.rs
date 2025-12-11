//! Player plugin
//!
//! Handles player entity spawning and related systems.

use bevy::prelude::*;

use crate::components::{
    AnimationClip, AnimationController, AnimationState, Collider, Gravity, GroundDetection, Player,
    PlayerStats, Velocity,
};
use crate::config::{
    SelectedCharacter, load_animation_config_optional, load_characters_config_optional,
};
use crate::game_state::GameState;
use crate::resources::CharacterAssets;
use crate::systems::{
    flip_sprite_by_facing, ground_collision, player_animation_controller, player_dash,
    player_horizontal_movement, player_jump, update_animations, update_dash, variable_jump_height,
    wall_collision, wall_jump, wall_slide,
};

/// Plugin that manages the player entity and systems
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
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
        // update_animations must run right after animation controller for immediate visual updates
        app.add_systems(
            FixedUpdate,
            (
                ground_collision,
                wall_collision,
                player_animation_controller,
                update_animations,
                flip_sprite_by_facing,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

/// Spawn the player entity
fn spawn_player(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    settings: Res<crate::config::GameSettings>,
    character_assets: Option<Res<CharacterAssets>>,
    selected_character: Option<Res<SelectedCharacter>>,
) {
    // Character display size (2x the sprite size for better visibility)
    let player_size = Vec2::new(64.0, 64.0);

    // Determine which character to use
    let character_id = selected_character
        .as_ref()
        .map(|sc| sc.character_id.as_str())
        .unwrap_or("fox");

    info!("Spawning player with character: {}", character_id);

    // Load character configuration
    let characters_config = load_characters_config_optional("assets/config/characters.ron");

    // Get animation config path from character definition
    let animation_config_path = if let Some(config) = characters_config {
        if let Ok(character) = config.get_character(character_id) {
            format!("assets/{}", character.animation_config_path)
        } else {
            warn!(
                "Character '{}' not found in characters.ron, using default",
                character_id
            );
            "assets/graphics/characters/players/fox/fox_animations.ron".to_string()
        }
    } else {
        warn!("Could not load characters.ron, using default fox animation config");
        "assets/graphics/characters/players/fox/fox_animations.ron".to_string()
    };

    // Try to load animation config
    let animation_config = load_animation_config_optional(&animation_config_path);

    // Create animation controller
    let mut animation_controller = if let Some(config) = animation_config {
        info!("Setting up player animations from config");
        AnimationController::from(config)
    } else {
        info!("Setting up placeholder player animations");
        // Create placeholder animations matching fox_animations.ron
        let mut controller = AnimationController::new();
        controller.add_animation("idle", AnimationClip::new(0, 3, 4.0));
        controller.add_animation("run", AnimationClip::new(6, 11, 12.0));
        controller.add_animation("climb", AnimationClip::new(12, 15, 10.0));
        controller.add_animation("crouch", AnimationClip::new(18, 20, 10.0));
        controller.add_animation("hurt", AnimationClip::new(24, 25, 15.0));
        controller.add_animation("jump", AnimationClip::new(30, 30, 1.0));
        controller.add_animation("fall", AnimationClip::new(31, 31, 1.0));
        controller.add_animation("dizzy", AnimationClip::new(48, 53, 8.0));
        controller.add_animation("roll", AnimationClip::new(54, 57, 12.0));
        controller.add_animation("look_up", AnimationClip::new(60, 60, 1.0));
        controller.add_animation("win", AnimationClip::new(66, 66, 1.0));
        controller.current_animation = "idle".to_string();
        controller
    };

    // Set up animation state
    let mut animation_state = AnimationState::new(10.0, true);

    // Explicitly play the initial animation to ensure timer is set correctly
    // Temporarily clear current_animation so play() will initialize it properly
    let initial_animation = animation_controller.current_animation.clone();
    animation_controller.current_animation = String::new();
    animation_controller.play(&initial_animation, &mut animation_state);

    // Check if we have character assets loaded
    if let Some(assets) = character_assets {
        // Spawn with texture atlas
        let mut entity = commands.spawn(Transform::from_xyz(0.0, 100.0, 0.0));

        entity.insert(Visibility::default());
        entity.insert(Sprite {
            image: assets.fox_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.fox_layout.clone(),
                index: 0,
            }),
            custom_size: Some(player_size),
            ..default()
        });
        entity.insert(Name::new("Player"));

        entity.insert(Player::default());
        entity.insert(PlayerStats::from(settings.player.clone()));
        entity.insert(GroundDetection::default());

        entity.insert(Velocity::zero());
        entity.insert(Gravity::default());
        entity.insert(Collider::new(player_size));

        entity.insert(animation_controller);
        entity.insert(animation_state);

        info!("Player spawned at position (0, 100) with texture atlas animation");
    } else {
        // Fallback to colored square if assets aren't loaded yet
        warn!("Character assets not loaded yet, spawning player with colored square");
        let mut entity = commands.spawn(Transform::from_xyz(0.0, 100.0, 0.0));

        entity.insert(Visibility::default());
        entity.insert(Sprite {
            color: Color::srgb(0.2, 0.7, 0.3), // Green placeholder
            custom_size: Some(player_size),
            ..default()
        });
        entity.insert(Name::new("Player"));

        entity.insert(Player::default());
        entity.insert(PlayerStats::from(settings.player.clone()));
        entity.insert(GroundDetection::default());

        entity.insert(Velocity::zero());
        entity.insert(Gravity::default());
        entity.insert(Collider::new(player_size));

        entity.insert(animation_controller);
        entity.insert(animation_state);

        info!("Player spawned at position (0, 100) with placeholder sprite");
    }
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
        crate::components::Ground,
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
        crate::components::Wall,
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
        crate::components::Wall,
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
        crate::components::Ground,
        Collider::new(platform_size),
        Name::new("Middle Platform"),
    ));

    info!("Test walls spawned at positions (-350, 50) and (350, 50)");
}
