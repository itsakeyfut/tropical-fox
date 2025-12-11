//! Game components
//!
//! This module will contain all ECS components used throughout the game.

use bevy::prelude::*;

/// Velocity component for entities that move
#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity with x and y components
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a zero velocity
    pub fn zero() -> Self {
        Self::default()
    }
}

/// Gravity component for entities affected by gravity
#[derive(Component, Debug)]
pub struct Gravity {
    /// Gravity scale multiplier (1.0 = normal gravity)
    pub scale: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

impl Gravity {
    /// Create a new gravity component with a custom scale
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
}
