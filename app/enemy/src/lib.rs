//! Enemy system for Tropical Fox
//!
//! This crate handles enemy AI, behaviors, and boss mechanics.

pub mod components;
pub mod config;
pub mod plugin;

// Re-export commonly used types
pub use components::*;
pub use config::*;
pub use plugin::EnemyPlugin;
