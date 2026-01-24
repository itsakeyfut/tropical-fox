//! Level-specific components
//!
//! Marker components for tiles and level entities

use bevy::prelude::*;

/// Marker for one-way platforms (can jump through from below)
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct OneWayPlatform;

/// Component for parallax background layers
#[derive(Component, Debug, Clone)]
pub struct ParallaxLayer {
    /// Parallax factor (0.0 = static, 1.0 = moves with camera)
    pub parallax_factor: f32,
    /// Automatic scroll speed (pixels per second)
    pub scroll_speed: Vec2,
    /// Initial position when spawned
    pub initial_position: Vec3,
}

impl ParallaxLayer {
    pub fn new(parallax_factor: f32, scroll_speed: Vec2) -> Self {
        Self {
            parallax_factor,
            scroll_speed,
            initial_position: Vec3::ZERO,
        }
    }
}

/// Marker for background layer entities
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct BackgroundLayer;

/// Marker for goal/level exit entities
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Goal;

/// Component for checkpoint entities
#[derive(Component, Debug, Clone)]
pub struct Checkpoint {
    /// Whether this checkpoint has been activated
    pub activated: bool,
    /// Spawn position for respawn
    pub spawn_position: Vec3,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            activated: false,
            spawn_position: Vec3::ZERO,
        }
    }
}

/// Resource tracking the currently active checkpoint
#[derive(Resource, Debug, Clone)]
pub struct ActiveCheckpoint {
    /// Position to respawn at
    pub respawn_position: Vec3,
}

impl Default for ActiveCheckpoint {
    fn default() -> Self {
        Self {
            respawn_position: Vec3::new(0.0, 100.0, 0.0), // Default spawn height
        }
    }
}
