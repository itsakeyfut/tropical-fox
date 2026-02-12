use bevy::prelude::*;

/// Resource tracking the active checkpoint for respawning
#[derive(Resource, Debug)]
pub struct ActiveCheckpoint {
    pub checkpoint_id: Option<String>,
    pub position: Vec3,
}

impl Default for ActiveCheckpoint {
    fn default() -> Self {
        Self {
            checkpoint_id: None,
            position: Vec3::new(0.0, 100.0, 0.0), // Default spawn
        }
    }
}

/// Resource tracking current level name
#[derive(Resource, Debug, Default)]
pub struct CurrentLevel {
    pub name: String,
}
