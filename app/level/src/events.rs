use bevy::prelude::*;

/// Event fired when player reaches level goal
#[derive(Message, Debug)]
pub struct LevelCompleteEvent {
    pub next_level: Option<String>,
}

/// Event fired when player activates a checkpoint
#[derive(Message, Debug)]
pub struct CheckpointActivatedEvent {
    pub checkpoint_id: String,
}
