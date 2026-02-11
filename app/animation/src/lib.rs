//! Animation system for Tropical Fox
//!
//! This crate handles sprite animations using benimator.

pub mod components;
pub mod config;
pub mod plugin;
pub mod systems;

// Re-export commonly used types
pub use components::*;
pub use config::*;
pub use plugin::AnimationPlugin;
