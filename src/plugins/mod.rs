//! Game plugins
//!
//! This module organizes game functionality into Bevy plugins.

pub mod animation;
pub mod core;
pub mod enemy;
pub mod level;
pub mod player;

pub use animation::AnimationPlugin;
pub use core::CorePlugin;
pub use enemy::EnemyPlugin;
pub use level::LevelPlugin;
pub use player::PlayerPlugin;
