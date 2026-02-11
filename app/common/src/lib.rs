//! Common types and utilities shared across all Tropical Fox crates
//!
//! This crate contains:
//! - Game state definitions
//! - Common ECS components
//! - Common events
//! - Common resources

pub mod components;
pub mod events;
pub mod game_state;
pub mod resources;

// Re-export commonly used types
pub use components::*;
pub use game_state::{GameState, InGameState};
pub use resources::*;
