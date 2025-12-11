//! Player plugin
//!
//! Handles player entity spawning and related systems.

use bevy::prelude::*;

use crate::components::{Collider, Gravity, GroundDetection, Player, PlayerStats, Velocity};
use crate::game_state::GameState;
use crate::systems::{
    flip_sprite_by_facing, ground_collision, player_horizontal_movement, player_jump,
};

/// Plugin that manages the player entity and systems
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Spawn player when entering InGame state
        app.add_systems(OnEnter(GameState::InGame), spawn_player);

        // Player input and movement systems (run in Update)
        app.add_systems(
            Update,
            (player_horizontal_movement, player_jump).run_if(in_state(GameState::InGame)),
        );

        // Collision and sprite systems (run in FixedUpdate after physics)
        app.add_systems(
            FixedUpdate,
            (ground_collision, flip_sprite_by_facing).run_if(in_state(GameState::InGame)),
        );
    }
}

/// Spawn the player entity
fn spawn_player(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // For now, use a simple colored square as placeholder sprite
    // TODO: Replace with actual player sprite once assets are available
    let player_size = Vec2::new(32.0, 32.0);

    commands.spawn((
        // Transform and visibility
        Transform::from_xyz(0.0, 100.0, 0.0),
        Visibility::default(),
        // Sprite rendering
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.3), // Green placeholder
            custom_size: Some(player_size),
            ..default()
        },
        // Player-specific components
        Player::default(),
        PlayerStats::default(),
        GroundDetection::default(),
        // Physics components
        Velocity::zero(),
        Gravity::default(),
        Collider::new(player_size),
        // Name for debugging
        Name::new("Player"),
    ));

    info!("Player spawned at position (0, 100)");
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
