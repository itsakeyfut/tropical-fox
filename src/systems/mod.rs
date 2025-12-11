//! Game systems
//!
//! This module will contain all ECS systems used throughout the game.

pub mod physics;

pub use physics::{apply_gravity, update_position};
