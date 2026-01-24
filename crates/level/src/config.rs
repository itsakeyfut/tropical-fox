//! Level configuration and management
//!
//! Handles level definitions, metadata, and background layers

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Background layer definition for parallax effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundLayerConfig {
    /// Path to the background sprite (relative to assets directory)
    pub sprite_path: String,
    /// Parallax factor (0.0 = static, 1.0 = moves with camera)
    #[serde(default = "default_parallax_factor")]
    pub parallax_factor: f32,
    /// Z-order for layering (lower values are further back)
    #[serde(default)]
    pub z_order: f32,
    /// Automatic scroll speed (pixels per second)
    #[serde(default)]
    pub scroll_speed: Vec2,
}

fn default_parallax_factor() -> f32 {
    0.5
}

/// Level metadata definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    /// Unique identifier for this level (e.g., "level_1", "boss_room_1")
    pub id: String,
    /// Display name for the level
    pub name: String,
    /// Path to the LDtk level file (relative to assets directory)
    pub ldtk_path: String,
    /// Background music asset path (optional)
    #[serde(default)]
    pub music_path: Option<String>,
    /// Background layers for parallax effect
    #[serde(default)]
    pub background_layers: Vec<BackgroundLayerConfig>,
    /// Level description
    #[serde(default)]
    pub description: String,
}

/// Levels configuration file format
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct LevelsConfig {
    /// Map of level_id -> level definition
    pub levels: HashMap<String, LevelConfig>,
    /// Default/starting level ID
    pub default_level: String,
}

/// Level configuration loading errors
#[derive(Debug, Error)]
pub enum LevelConfigError {
    #[error("Failed to read level config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON level config: {0}")]
    RonError(#[from] ron::error::SpannedError),

    #[error("Level not found: {0}")]
    LevelNotFound(String),
}

/// Load levels configuration from a RON file
pub fn load_levels_config<P: AsRef<Path>>(path: P) -> Result<LevelsConfig, LevelConfigError> {
    let content = fs::read_to_string(path)?;
    let config: LevelsConfig = ron::from_str(&content)?;
    Ok(config)
}

/// Load levels configuration from a RON file, or return None if loading/parsing fails
pub fn load_levels_config_optional<P: AsRef<Path>>(path: P) -> Option<LevelsConfig> {
    match load_levels_config(path) {
        Ok(config) => {
            info!("Loaded levels config with {} levels", config.levels.len());
            Some(config)
        }
        Err(e) => {
            warn!("Failed to load levels config: {}", e);
            None
        }
    }
}

impl LevelsConfig {
    /// Get a level definition by ID
    pub fn get_level(&self, id: &str) -> Result<&LevelConfig, LevelConfigError> {
        self.levels
            .get(id)
            .ok_or_else(|| LevelConfigError::LevelNotFound(id.to_string()))
    }

    /// Get the default level
    pub fn get_default_level(&self) -> Result<&LevelConfig, LevelConfigError> {
        self.get_level(&self.default_level)
    }

    /// Get all level definitions
    pub fn get_all_levels(&self) -> impl Iterator<Item = &LevelConfig> {
        self.levels.values()
    }
}

/// Resource that tracks the currently active level
#[derive(Resource, Debug, Clone)]
pub struct CurrentLevel {
    /// Currently active level ID
    pub level_id: String,
}

impl CurrentLevel {
    /// Create a new current level tracker
    pub fn new(level_id: impl Into<String>) -> Self {
        Self {
            level_id: level_id.into(),
        }
    }
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            level_id: "test_level".to_string(),
        }
    }
}
