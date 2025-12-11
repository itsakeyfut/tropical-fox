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

        // Register startup systems
        app.add_systems(Startup, setup_camera);

        // Register physics systems (run in FixedUpdate for consistent physics)
        app.add_systems(FixedUpdate, (apply_gravity, update_position).chain());
    }
}

/// Set up the main 2D camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Name::new("Main Camera")));
}
