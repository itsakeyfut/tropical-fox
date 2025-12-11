//! Game plugins
//!
//! This module organizes game functionality into Bevy plugins.

pub mod core;
pub mod player;

pub use core::CorePlugin;
pub use player::PlayerPlugin;
