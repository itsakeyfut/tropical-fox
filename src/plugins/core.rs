//! Core game plugin
//!
//! Handles basic game initialization and setup.

use bevy::prelude::*;

use crate::config;
use crate::game_state::{GameState, InGameState};
use crate::resources::PhysicsConfig;
use crate::systems::{apply_gravity, update_position};

/// Core plugin that sets up fundamental game systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Initialize game states
        app.init_state::<GameState>().add_sub_state::<InGameState>();

        // Load game settings
        let settings = config::load_settings_or_default("assets/config/game_settings.ron");

        // Initialize physics configuration from settings
        let physics_config = PhysicsConfig {
            gravity: settings.physics.gravity,
            terminal_velocity: settings.physics.terminal_velocity,
        };

        app.insert_resource(physics_config);
        app.insert_resource(settings);

        // Register startup systems
        app.add_systems(Startup, (setup_camera, setup_initial_state));

        // Register physics systems (run in FixedUpdate for consistent physics)
        app.add_systems(FixedUpdate, (apply_gravity, update_position).chain());
    }
}

/// Set up the main 2D camera with zoom for pixel art
fn setup_camera(mut commands: Commands, settings: Res<config::GameSettings>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // Zoom in to make tiles more visible (smaller scale = more zoom)
            // Configured via assets/config/game_settings.ron
            scale: settings.display.camera_scale,
            ..OrthographicProjection::default_2d()
        }),
        Name::new("Main Camera"),
    ));
}

/// Set initial game state to InGame for development
fn setup_initial_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}
