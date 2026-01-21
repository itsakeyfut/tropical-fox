//! Game events
//!
//! This module contains all event types used throughout the game.

use bevy::prelude::*;

// ============================================================================
// Level Events
// ============================================================================

/// Event fired when the player reaches the goal
#[derive(Message, Debug)]
pub struct GoalReachedEvent;

/// Event fired when the player activates a checkpoint
#[derive(Message, Debug)]
pub struct CheckpointActivatedEvent {
    /// The unique ID of the activated checkpoint
    pub checkpoint_id: u32,
    /// The world position of the checkpoint
    pub position: Vec2,
}

/// Event fired when a level finishes loading
#[derive(Message, Debug)]
pub struct LevelLoadedEvent {
    /// The level index that was loaded
    pub level_index: usize,
}
