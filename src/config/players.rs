//! Player configuration and management
//!
//! Handles player character definitions and selection

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Player character definition with asset paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTypeConfig {
    /// Unique identifier for this character (e.g., "fox", "rabbit")
    pub id: String,
    /// Display name for the character
    pub name: String,
    /// Path to the animation config file (relative to assets directory)
    pub animation_config_path: String,
    /// Description of the character
    #[serde(default)]
    pub description: String,
}

/// Players configuration file format
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct PlayersConfig {
    /// Map of player_id -> player definition
    pub players: HashMap<String, PlayerTypeConfig>,
    /// Default player character ID
    pub default_player: String,
}

/// Player configuration loading errors
#[derive(Debug, Error)]
pub enum PlayerConfigError {
    #[error("Failed to read player config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON player config: {0}")]
    RonError(#[from] ron::error::SpannedError),

    #[error("Player not found: {0}")]
    PlayerNotFound(String),
}

/// Load players configuration from a RON file
pub fn load_players_config<P: AsRef<Path>>(path: P) -> Result<PlayersConfig, PlayerConfigError> {
    let content = fs::read_to_string(path)?;
    let config: PlayersConfig = ron::from_str(&content)?;
    Ok(config)
}

/// Load players configuration from a RON file, or return default if loading/parsing fails
pub fn load_players_config_optional<P: AsRef<Path>>(path: P) -> Option<PlayersConfig> {
    match load_players_config(path) {
        Ok(config) => {
            info!(
                "Loaded players config with {} players",
                config.players.len()
            );
            Some(config)
        }
        Err(e) => {
            warn!("Failed to load players config: {}", e);
            None
        }
    }
}

impl PlayersConfig {
    /// Get a player definition by ID
    pub fn get_player(&self, id: &str) -> Result<&PlayerTypeConfig, PlayerConfigError> {
        self.players
            .get(id)
            .ok_or_else(|| PlayerConfigError::PlayerNotFound(id.to_string()))
    }

    /// Get the default player character
    pub fn get_default_player(&self) -> Result<&PlayerTypeConfig, PlayerConfigError> {
        self.get_player(&self.default_player)
    }

    /// Get all player definitions
    pub fn get_all_players(&self) -> impl Iterator<Item = &PlayerTypeConfig> {
        self.players.values()
    }
}

/// Resource that tracks the currently selected player character
#[derive(Resource, Debug, Clone)]
pub struct SelectedCharacter {
    /// Currently selected character ID
    pub character_id: String,
}

impl SelectedCharacter {
    /// Create a new selected character
    pub fn new(character_id: impl Into<String>) -> Self {
        Self {
            character_id: character_id.into(),
        }
    }
}

impl Default for SelectedCharacter {
    fn default() -> Self {
        Self::new("fox")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_definition() {
        let player = PlayerTypeConfig {
            id: "fox".to_string(),
            name: "Fox".to_string(),
            animation_config_path: "graphics/characters/players/fox/fox_animations.ron".to_string(),
            description: "A clever fox character".to_string(),
        };

        assert_eq!(player.id, "fox");
        assert_eq!(player.name, "Fox");
    }

    #[test]
    fn test_players_config() {
        let mut players = HashMap::new();
        players.insert(
            "fox".to_string(),
            PlayerTypeConfig {
                id: "fox".to_string(),
                name: "Fox".to_string(),
                animation_config_path: "graphics/characters/players/fox/fox_animations.ron"
                    .to_string(),
                description: "A clever fox".to_string(),
            },
        );

        let config = PlayersConfig {
            players,
            default_player: "fox".to_string(),
        };

        assert!(config.get_player("fox").is_ok());
        assert!(config.get_player("unknown").is_err());
        assert!(config.get_default_player().is_ok());
        assert_eq!(config.get_all_players().count(), 1);
    }
}
