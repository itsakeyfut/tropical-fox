//! Character configuration and management
//!
//! Handles character definitions and selection

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Character type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterType {
    /// Playable characters
    Player,
    /// Enemy characters
    Enemy,
    /// NPC characters
    NPC,
}

/// Character definition with asset paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDefinition {
    /// Unique identifier for this character (e.g., "fox", "rabbit", "slime")
    pub id: String,
    /// Display name for the character
    pub name: String,
    /// Character type (player, enemy, npc)
    pub character_type: CharacterType,
    /// Path to the animation config file (relative to assets directory)
    pub animation_config_path: String,
    /// Description of the character
    #[serde(default)]
    pub description: String,
}

/// Characters configuration file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharactersConfig {
    /// Map of character ID to character definition
    pub characters: HashMap<String, CharacterDefinition>,
    /// Default player character ID
    pub default_player: String,
}

/// Character configuration loading errors
#[derive(Debug, Error)]
pub enum CharacterConfigError {
    #[error("Failed to read character config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON character config: {0}")]
    RonError(#[from] ron::error::SpannedError),

    #[error("Character not found: {0}")]
    CharacterNotFound(String),
}

/// Load characters configuration from a RON file
pub fn load_characters_config<P: AsRef<Path>>(
    path: P,
) -> Result<CharactersConfig, CharacterConfigError> {
    let content = fs::read_to_string(path)?;
    let config: CharactersConfig = ron::from_str(&content)?;
    Ok(config)
}

/// Load characters configuration from a RON file, or return None if file doesn't exist
pub fn load_characters_config_optional<P: AsRef<Path>>(path: P) -> Option<CharactersConfig> {
    match load_characters_config(path) {
        Ok(config) => Some(config),
        Err(e) => {
            warn!("Failed to load characters config: {}", e);
            None
        }
    }
}

impl CharactersConfig {
    /// Get a character definition by ID
    pub fn get_character(&self, id: &str) -> Result<&CharacterDefinition, CharacterConfigError> {
        self.characters
            .get(id)
            .ok_or_else(|| CharacterConfigError::CharacterNotFound(id.to_string()))
    }

    /// Get the default player character
    pub fn get_default_player(&self) -> Result<&CharacterDefinition, CharacterConfigError> {
        self.get_character(&self.default_player)
    }

    /// Get all playable characters
    pub fn get_playable_characters(&self) -> Vec<&CharacterDefinition> {
        self.characters
            .values()
            .filter(|c| c.character_type == CharacterType::Player)
            .collect()
    }

    /// Get all enemy characters
    pub fn get_enemy_characters(&self) -> Vec<&CharacterDefinition> {
        self.characters
            .values()
            .filter(|c| c.character_type == CharacterType::Enemy)
            .collect()
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
    fn test_character_definition() {
        let character = CharacterDefinition {
            id: "fox".to_string(),
            name: "Fox".to_string(),
            character_type: CharacterType::Player,
            animation_config_path: "graphics/characters/players/fox/fox_animations.ron".to_string(),
            description: "A clever fox character".to_string(),
        };

        assert_eq!(character.id, "fox");
        assert_eq!(character.character_type, CharacterType::Player);
    }

    #[test]
    fn test_characters_config() {
        let mut characters = HashMap::new();
        characters.insert(
            "fox".to_string(),
            CharacterDefinition {
                id: "fox".to_string(),
                name: "Fox".to_string(),
                character_type: CharacterType::Player,
                animation_config_path: "graphics/characters/players/fox/fox_animations.ron"
                    .to_string(),
                description: "A clever fox".to_string(),
            },
        );

        let config = CharactersConfig {
            characters,
            default_player: "fox".to_string(),
        };

        assert!(config.get_character("fox").is_ok());
        assert!(config.get_default_player().is_ok());
        assert_eq!(config.get_playable_characters().len(), 1);
    }
}
