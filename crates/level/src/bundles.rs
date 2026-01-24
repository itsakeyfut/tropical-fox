//! LDtk entity and IntGrid cell bundles
//!
//! Bundles for spawning tiles from LDtk IntGrid values

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use tropical_fox_common::{Collider, Ground, Wall};

use crate::components::{Checkpoint, Goal, OneWayPlatform};

/// Bundle for solid ground tiles (IntGrid value 1)
#[derive(Bundle, LdtkIntCell)]
pub struct SolidGroundBundle {
    ground: Ground,
}

impl Default for SolidGroundBundle {
    fn default() -> Self {
        Self { ground: Ground }
    }
}

impl From<&EntityInstance> for SolidGroundBundle {
    fn from(_entity_instance: &EntityInstance) -> Self {
        Self::default()
    }
}

impl From<IntGridCell> for SolidGroundBundle {
    fn from(_int_grid_cell: IntGridCell) -> Self {
        Self { ground: Ground }
    }
}

/// Bundle for solid wall tiles (IntGrid value 2)
#[derive(Bundle, LdtkIntCell)]
pub struct SolidWallBundle {
    wall: Wall,
}

impl Default for SolidWallBundle {
    fn default() -> Self {
        Self { wall: Wall }
    }
}

impl From<&EntityInstance> for SolidWallBundle {
    fn from(_entity_instance: &EntityInstance) -> Self {
        Self::default()
    }
}

impl From<IntGridCell> for SolidWallBundle {
    fn from(_int_grid_cell: IntGridCell) -> Self {
        Self::default()
    }
}

/// Bundle for one-way platform tiles (IntGrid value 3)
#[derive(Bundle, LdtkIntCell)]
pub struct OneWayPlatformBundle {
    ground: Ground,
    one_way: OneWayPlatform,
}

impl Default for OneWayPlatformBundle {
    fn default() -> Self {
        Self {
            ground: Ground,
            one_way: OneWayPlatform,
        }
    }
}

impl From<&EntityInstance> for OneWayPlatformBundle {
    fn from(_entity_instance: &EntityInstance) -> Self {
        Self::default()
    }
}

impl From<IntGridCell> for OneWayPlatformBundle {
    fn from(_int_grid_cell: IntGridCell) -> Self {
        Self::default()
    }
}

/// Bundle for goal entities (LDtk entity)
#[derive(Bundle, LdtkEntity)]
pub struct GoalBundle {
    goal: Goal,
    collider: Collider,
}

impl Default for GoalBundle {
    fn default() -> Self {
        Self {
            goal: Goal,
            collider: Collider::new(Vec2::new(32.0, 32.0)),
        }
    }
}

impl From<&EntityInstance> for GoalBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        let size = Vec2::new(entity_instance.width as f32, entity_instance.height as f32);

        Self {
            goal: Goal,
            collider: Collider::new(size),
        }
    }
}

/// Bundle for checkpoint entities (LDtk entity)
#[derive(Bundle, LdtkEntity)]
pub struct CheckpointBundle {
    checkpoint: Checkpoint,
    collider: Collider,
}

impl Default for CheckpointBundle {
    fn default() -> Self {
        Self {
            checkpoint: Checkpoint::default(),
            collider: Collider::new(Vec2::new(32.0, 32.0)),
        }
    }
}

impl From<&EntityInstance> for CheckpointBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        let size = Vec2::new(entity_instance.width as f32, entity_instance.height as f32);

        let spawn_position = Vec3::new(
            entity_instance.px.x as f32,
            entity_instance.px.y as f32,
            0.0,
        );

        Self {
            checkpoint: Checkpoint {
                activated: false,
                spawn_position,
            },
            collider: Collider::new(size),
        }
    }
}
