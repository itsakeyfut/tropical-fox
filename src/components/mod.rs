//! Game components
//!
//! This module will contain all ECS components used throughout the game.

pub mod animation;
pub mod enemy;

use bevy::prelude::*;

pub use animation::{
    AnimationClip, AnimationController, AnimationError, AnimationEvent, AnimationEvents,
    AnimationState,
};
pub use enemy::{
    ChaseAI, ContactDamage, Enemy, EnemyAI, EnemyProjectile, EnemyStats, FlyingAI, FlyingPattern,
    PatrolAI, ProjectileShooter,
};

/// Velocity component for entities that move
#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity with x and y components
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
}

/// Player marker component with facing direction
#[derive(Component, Debug)]
pub struct Player {
    /// True if player is facing right, false if facing left
    pub facing_right: bool,
    /// Coyote time remaining (allows jump shortly after leaving ground)
    pub coyote_timer: f32,
    /// Jump buffer timer (remembers jump input before landing)
    pub jump_buffer_timer: f32,
    /// Number of dashes remaining (resets when grounded)
    pub dashes_remaining: u32,
    /// Dash timer - counts down during dash
    pub dash_timer: f32,
    /// Dash direction (normalized)
    pub dash_direction: Vec2,
    /// Whether player is touching a wall on the left (-1), right (1), or none (0)
    pub wall_contact: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            facing_right: true,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            dashes_remaining: 1, // Start with 1 dash available
            dash_timer: 0.0,
            dash_direction: Vec2::ZERO,
            wall_contact: 0,
        }
    }
}

/// Player movement and physics statistics
#[derive(Component, Debug)]
pub struct PlayerStats {
    /// Maximum horizontal movement speed (pixels per second)
    pub move_speed: f32,
    /// Horizontal acceleration rate (higher = more responsive)
    /// Multiplied by delta time to create frame-rate independent interpolation
    /// Typical range: 5.0-20.0
    pub acceleration: f32,
    /// Horizontal deceleration rate when on ground (higher = more responsive)
    /// Multiplied by delta time to create frame-rate independent interpolation
    /// Typical range: 5.0-20.0
    pub deceleration: f32,
    /// Horizontal deceleration rate when in air (air friction)
    /// Higher values = more responsive air control and better landing precision
    pub air_deceleration: f32,
    /// Jump force (upward velocity in pixels per second)
    pub jump_force: f32,
    /// Coyote time duration (seconds) - time after leaving ground where jump is still allowed
    /// Celeste uses ~0.1 seconds (6 frames at 60fps)
    pub coyote_time: f32,
    /// Jump buffer duration (seconds) - time before landing where jump input is remembered
    /// Celeste uses ~0.1 seconds
    pub jump_buffer_time: f32,
    /// Jump cut multiplier - reduces upward velocity when jump button is released early
    /// Typical range: 0.3-0.5 (lower = more dramatic cut)
    pub jump_cut_multiplier: f32,
    /// Dash speed (pixels per second)
    pub dash_speed: f32,
    /// Dash duration (seconds)
    pub dash_duration: f32,
    /// Maximum number of air dashes before needing to touch ground
    pub max_air_dashes: u32,
    /// Wall jump horizontal force (pixels per second)
    pub wall_jump_force_x: f32,
    /// Wall jump vertical force (pixels per second)
    pub wall_jump_force_y: f32,
    /// Wall slide speed (pixels per second, negative = downward)
    pub wall_slide_speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            acceleration: 10.0,
            deceleration: 15.0,
            air_deceleration: 20.0,
            jump_force: 400.0,
            coyote_time: 0.15,        // Slightly more forgiving than Celeste
            jump_buffer_time: 0.15,   // Slightly more forgiving than Celeste
            jump_cut_multiplier: 0.4, // Cuts jump to 40% when released early
            dash_speed: 500.0,
            dash_duration: 0.15,
            max_air_dashes: 1,
            wall_jump_force_x: 300.0,
            wall_jump_force_y: 400.0,
            wall_slide_speed: -100.0,
        }
    }
}

impl From<crate::config::PlayerSettings> for PlayerStats {
    fn from(settings: crate::config::PlayerSettings) -> Self {
        Self {
            move_speed: settings.move_speed,
            acceleration: settings.acceleration,
            deceleration: settings.deceleration,
            air_deceleration: settings.air_deceleration,
            jump_force: settings.jump_force,
            coyote_time: settings.coyote_time,
            jump_buffer_time: settings.jump_buffer_time,
            jump_cut_multiplier: settings.jump_cut_multiplier,
            dash_speed: settings.dash_speed,
            dash_duration: settings.dash_duration,
            max_air_dashes: settings.max_air_dashes,
            wall_jump_force_x: settings.wall_jump_force_x,
            wall_jump_force_y: settings.wall_jump_force_y,
            wall_slide_speed: settings.wall_slide_speed,
        }
    }
}

/// Ground detection for jump mechanics
#[derive(Component, Debug)]
pub struct GroundDetection {
    /// Whether the entity is currently on the ground
    pub is_grounded: bool,
    /// Distance to check below entity for ground (pixels)
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn with_offset(size: Vec2, offset: Vec2) -> Self {
        Self { size, offset }
    }
}

/// Ground marker component for platforms and surfaces
#[derive(Component, Debug)]
pub struct Ground;

/// Wall marker component for vertical surfaces that can be wall-jumped
#[derive(Component, Debug)]
pub struct Wall;
