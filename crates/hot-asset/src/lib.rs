//! Hot-reload system for RON configuration files
//!
//! This module provides a reusable hot-reload system that integrates with
//! Bevy's asset system to automatically reload configuration files when they
//! change on disk.
//!
//! ## Features
//! - 🔥 Hot-reload any RON config without restarting
//! - 🛡️ Graceful error handling (keeps previous valid value)
//! - 📊 Optional metrics and reload history
//! - 🎯 Type-safe with Rust's type system
//! - ⚡ Debouncing to prevent reload storms
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use hot_asset::{HotReloadPlugin, HotAssetHandle, AssetReloaded};
//!
//! #[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
//! struct GameSettings {
//!     gravity: f32,
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(HotReloadPlugin::<GameSettings>::new("config/game_settings.ron"))
//!         .run();
//! }
//! ```

pub mod bevy_adapter;
pub mod core;

// Re-export commonly used types
pub use bevy_adapter::{AssetReloaded, HotAssetHandle, HotReloadPlugin, ReloadMetrics};
