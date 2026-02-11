//! Player system for Tropical Fox
//!
//! This crate handles player controls, movement, and physics.

pub mod config;
pub mod plugin;
pub mod systems;

// Re-export commonly used types
pub use config::*;
pub use plugin::{PlayerPlugin, spawn_test_ground};
