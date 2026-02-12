use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::{Checkpoint, LevelGoal};

/// LDtk entity bundle for checkpoints
#[derive(Bundle, LdtkEntity, Default)]
pub struct CheckpointBundle {
    checkpoint: Checkpoint,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            checkpoint_id: "default".to_string(),
            activated: false,
        }
    }
}

impl From<&EntityInstance> for Checkpoint {
    fn from(entity_instance: &EntityInstance) -> Self {
        // Extract checkpoint_id from LDtk entity fields
        let checkpoint_id = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "checkpoint_id")
            .and_then(|f| match &f.value {
                FieldValue::String(Some(id)) => Some(id.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "default".to_string());

        Checkpoint {
            checkpoint_id,
            activated: false,
        }
    }
}

/// LDtk entity bundle for level goals
#[derive(Bundle, LdtkEntity, Default)]
pub struct GoalBundle {
    goal: LevelGoal,
}

impl From<&EntityInstance> for LevelGoal {
    fn from(entity_instance: &EntityInstance) -> Self {
        // Extract next_level from LDtk entity fields
        let next_level = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "next_level")
            .and_then(|f| match &f.value {
                FieldValue::String(Some(level)) if !level.is_empty() => Some(level.clone()),
                _ => None,
            });

        LevelGoal { next_level }
    }
}
