//! Game resources
//!
//! This module will contain all ECS resources used throughout the game.

use bevy::prelude::*;

/// Physics configuration resource
#[derive(Resource, Debug)]
pub struct PhysicsConfig {
    /// Base gravity acceleration (pixels per second squared)
    /// Negative value pulls entities downward
    pub gravity: f32,
    /// Terminal velocity (maximum falling speed)
    pub terminal_velocity: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: -980.0, // Standard gravity in pixels/s²
            terminal_velocity: -500.0,
        }
    }
}
