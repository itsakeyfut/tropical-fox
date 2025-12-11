//! Core game plugin
//!
//! Handles basic game initialization and setup.

use bevy::prelude::*;

use crate::game_state::{GameState, InGameState};

/// Core plugin that sets up fundamental game systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_sub_state::<InGameState>();
    }
}
