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

/// Player marker component with facing direction
#[derive(Component, Debug)]
pub struct Player {
    /// True if player is facing right, false if facing left
    pub facing_right: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self { facing_right: true }
    }
}

/// Player movement and physics statistics
#[derive(Component, Debug)]
pub struct PlayerStats {
    /// Maximum horizontal movement speed (pixels per second)
    pub move_speed: f32,
    /// Horizontal acceleration rate (0.0 to 1.0, lower = more responsive)
    pub acceleration: f32,
    /// Horizontal deceleration rate (0.0 to 1.0, lower = more responsive)
    pub deceleration: f32,
    /// Jump force (upward velocity in pixels per second)
    pub jump_force: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            acceleration: 10.0,
            deceleration: 15.0,
            jump_force: 400.0,
        }
    }
}

/// Ground detection for jump mechanics
#[derive(Component, Debug)]
pub struct GroundDetection {
    /// Whether the entity is currently on the ground
    pub is_grounded: bool,
    /// Distance to check below entity for ground (pixels)
    pub ground_check_distance: f32,
}

impl Default for GroundDetection {
    fn default() -> Self {
        Self {
            is_grounded: false,
            ground_check_distance: 2.0,
        }
    }
}

/// AABB (Axis-Aligned Bounding Box) collider
#[derive(Component, Debug)]
pub struct Collider {
    /// Size of the collider (width, height)
    pub size: Vec2,
    /// Offset from the entity's transform position
    pub offset: Vec2,
}

impl Collider {
    /// Create a new collider with given size and no offset
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            offset: Vec2::ZERO,
        }
    }

    /// Create a new collider with given size and offset
    pub fn with_offset(size: Vec2, offset: Vec2) -> Self {
        Self { size, offset }
    }
}

/// Ground marker component for platforms and surfaces
#[derive(Component, Debug)]
pub struct Ground;
