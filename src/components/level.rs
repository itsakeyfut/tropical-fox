//! Level-related components
//!
//! Components for tilemap collision, goals, checkpoints, and parallax backgrounds.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::{Collider, Ground, Wall};

// ============================================================================
// Collision Components
// ============================================================================

/// Marker component for collision tiles
#[derive(Default, Component, Debug)]
pub struct CollisionTile;

/// Bundle for ground collision tiles (IntGrid value 1)
#[derive(Default, Bundle, LdtkIntCell)]
pub struct GroundCollisionBundle {
    pub collision: CollisionTile,
    pub ground: Ground,
}

/// Bundle for wall collision tiles (IntGrid value 2)
#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallCollisionBundle {
    pub collision: CollisionTile,
    pub wall: Wall,
}

// ============================================================================
// Goal Components
// ============================================================================

/// Marker component for the level goal
#[derive(Default, Component, Debug)]
pub struct Goal;

/// Bundle for goal entities in LDtk
#[derive(Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    pub goal: Goal,
    #[worldly]
    pub worldly: Worldly,
}

// ============================================================================
// Checkpoint Components
// ============================================================================

/// Component for checkpoint entities
#[derive(Component, Debug)]
pub struct Checkpoint {
    /// Unique identifier for this checkpoint
    pub id: u32,
    /// Whether this checkpoint has been activated
    pub activated: bool,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            id: 0,
            activated: false,
        }
    }
}

/// Bundle for checkpoint entities in LDtk
#[derive(Default, Bundle, LdtkEntity)]
pub struct CheckpointBundle {
    pub checkpoint: Checkpoint,
    #[worldly]
    pub worldly: Worldly,
}

// ============================================================================
// Player Spawn Components
// ============================================================================

/// Marker component for player spawn point
#[derive(Default, Component, Debug)]
pub struct PlayerSpawnPoint;

/// Bundle for player spawn entities in LDtk
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    pub spawn_point: PlayerSpawnPoint,
    #[worldly]
    pub worldly: Worldly,
}

// ============================================================================
// Parallax Components
// ============================================================================

/// Component for parallax background layers
#[derive(Component, Debug)]
pub struct ParallaxLayer {
    /// Parallax speed factor (0.0 = static, 1.0 = moves with camera)
    /// Lower values create more distant/slower moving backgrounds
    pub speed: f32,
    /// Layer index for z-ordering (higher = closer to camera)
    pub layer_index: u32,
    /// Original X position for calculating parallax offset
    pub base_x: f32,
    /// Original Y position for calculating parallax offset
    pub base_y: f32,
}

impl ParallaxLayer {
    /// Create a new parallax layer
    pub fn new(speed: f32, layer_index: u32) -> Self {
        Self {
            speed,
            layer_index,
            base_x: 0.0,
            base_y: 0.0,
        }
    }

    /// Create a new parallax layer with base position
    pub fn with_position(speed: f32, layer_index: u32, base_x: f32, base_y: f32) -> Self {
        Self {
            speed,
            layer_index,
            base_x,
            base_y,
        }
    }
}

impl Default for ParallaxLayer {
    fn default() -> Self {
        Self {
            speed: 0.5,
            layer_index: 0,
            base_x: 0.0,
            base_y: 0.0,
        }
    }
}

// ============================================================================
// Level Marker Components
// ============================================================================

/// Marker component for all level-related entities (used for cleanup)
#[derive(Default, Component, Debug)]
pub struct LevelEntity;

/// Marker component for the current loaded level
#[derive(Default, Component, Debug)]
pub struct CurrentLevelMarker;
