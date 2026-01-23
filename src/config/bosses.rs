//! Boss configuration and management
//!
//! Handles boss character definitions

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Boss character definition with asset paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossTypeConfig {
    /// Unique identifier for this boss (e.g., "sunny_dragon")
    pub id: String,
    /// Display name for the boss
    pub name: String,
    /// Path to the animation config file (relative to assets directory)
    pub animation_config_path: String,
    /// Description of the boss
    #[serde(default)]
    pub description: String,
}

/// Bosses configuration file format
#[derive(Debug, Clone, Serialize, Deserialize, Default, Asset, TypePath)]
pub struct BossesConfig {
    /// Map of boss_id -> boss definition
    pub bosses: HashMap<String, BossTypeConfig>,
}

/// Boss configuration loading errors
#[derive(Debug, Error)]
pub enum BossConfigError {
    #[error("Failed to read boss config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON boss config: {0}")]
    RonError(#[from] ron::error::SpannedError),

    #[error("Boss not found: {0}")]
    BossNotFound(String),
}

/// Load bosses configuration from a RON file
pub fn load_bosses_config<P: AsRef<Path>>(path: P) -> Result<BossesConfig, BossConfigError> {
    let content = fs::read_to_string(path)?;
    let config: BossesConfig = ron::from_str(&content)?;
    Ok(config)
}

/// Load bosses configuration from a RON file, or return default if loading/parsing fails
pub fn load_bosses_config_optional<P: AsRef<Path>>(path: P) -> BossesConfig {
    match load_bosses_config(path) {
        Ok(config) => {
            info!("Loaded bosses config with {} bosses", config.bosses.len());
            config
        }
        Err(e) => {
            warn!("Failed to load bosses config: {}. Using default.", e);
            BossesConfig::default()
        }
    }
}

impl BossesConfig {
    /// Get a boss definition by ID
    pub fn get_boss(&self, id: &str) -> Result<&BossTypeConfig, BossConfigError> {
        self.bosses
            .get(id)
            .ok_or_else(|| BossConfigError::BossNotFound(id.to_string()))
    }

    /// Get all boss definitions
    pub fn get_all_bosses(&self) -> impl Iterator<Item = &BossTypeConfig> {
        self.bosses.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boss_definition() {
        let boss = BossTypeConfig {
            id: "sunny_dragon".to_string(),
            name: "Sunny Dragon".to_string(),
            animation_config_path:
                "graphics/characters/bosses/sunny_dragon/sunny_dragon_animations.ron".to_string(),
            description: "A fearsome dragon".to_string(),
        };

        assert_eq!(boss.id, "sunny_dragon");
        assert_eq!(boss.name, "Sunny Dragon");
    }

    #[test]
    fn test_bosses_config() {
        let mut bosses = HashMap::new();
        bosses.insert(
            "sunny_dragon".to_string(),
            BossTypeConfig {
                id: "sunny_dragon".to_string(),
                name: "Sunny Dragon".to_string(),
                animation_config_path:
                    "graphics/characters/bosses/sunny_dragon/sunny_dragon_animations.ron"
                        .to_string(),
                description: "A fearsome dragon".to_string(),
            },
        );

        let config = BossesConfig { bosses };

        assert!(config.get_boss("sunny_dragon").is_ok());
        assert!(config.get_boss("unknown").is_err());
        assert_eq!(config.get_all_bosses().count(), 1);
    }

    #[test]
    fn test_bosses_config_default() {
        let config = BossesConfig::default();
        assert!(config.bosses.is_empty());
    }
}
