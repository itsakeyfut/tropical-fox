//! Game state definitions and transitions

use bevy::prelude::*;

/// Top-level game state
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// Initial loading state
    #[default]
    Loading,
    /// Title screen
    Title,
    /// World map navigation
    WorldMap,
    /// Active gameplay
    InGame,
    /// Paused game
    Paused,
    /// Game over screen
    GameOver,
}

/// Sub-state when in active gameplay
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    /// Normal stage gameplay
    #[default]
    StagePlay,
    /// Boss battle
    BossRoom,
    /// Transitioning between stages
    StageTransition,
}
