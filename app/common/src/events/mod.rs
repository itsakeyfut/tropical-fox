//! Game events
//!
//! This module will contain all event types used throughout the game.

use bevy::prelude::*;

/// Event fired when the player dies
#[derive(Message, Debug)]
pub struct PlayerDeathEvent;
