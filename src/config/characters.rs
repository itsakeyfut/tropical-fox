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
    /// Boss characters
    Boss,
    /// Enemy characters
    Enemy,
    /// NPC characters
    Npc,
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
///
/// The structure is organized by category:
/// - "players": playable characters
/// - "bosses": boss characters
/// - "enemies": enemy characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharactersConfig {
    /// Nested map: category -> character_id -> character definition
    pub characters: HashMap<String, HashMap<String, CharacterDefinition>>,
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

    #[error("Duplicate character ID '{id}' found in categories: {categories}")]
    DuplicateCharacterId { id: String, categories: String },
}

/// Load characters configuration from a RON file
///
/// This function validates that all character IDs are unique across all categories.
/// If duplicate IDs are found, an error is returned.
pub fn load_characters_config<P: AsRef<Path>>(
    path: P,
) -> Result<CharactersConfig, CharacterConfigError> {
    let content = fs::read_to_string(path)?;
    let config: CharactersConfig = ron::from_str(&content)?;

    // Validate that all character IDs are unique across categories
    config.validate_unique_ids()?;

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
    /// Validate that all character IDs are unique across all categories
    ///
    /// Returns an error if any character ID appears in multiple categories.
    pub fn validate_unique_ids(&self) -> Result<(), CharacterConfigError> {
        // Track which categories each ID appears in
        let mut id_to_categories: HashMap<&str, Vec<&str>> = HashMap::new();

        for (category, characters) in &self.characters {
            for character_id in characters.keys() {
                id_to_categories
                    .entry(character_id.as_str())
                    .or_default()
                    .push(category.as_str());
            }
        }

        // Check for duplicates
        for (id, categories) in id_to_categories {
            if categories.len() > 1 {
                return Err(CharacterConfigError::DuplicateCharacterId {
                    id: id.to_string(),
                    categories: categories.join(", "),
                });
            }
        }

        Ok(())
    }

    /// Get a character definition by ID (searches all categories)
    pub fn get_character(&self, id: &str) -> Result<&CharacterDefinition, CharacterConfigError> {
        for category in self.characters.values() {
            if let Some(character) = category.get(id) {
                return Ok(character);
            }
        }
        Err(CharacterConfigError::CharacterNotFound(id.to_string()))
    }

    /// Get the default player character
    pub fn get_default_player(&self) -> Result<&CharacterDefinition, CharacterConfigError> {
        self.get_character(&self.default_player)
    }

    /// Get all playable characters
    pub fn get_playable_characters(&self) -> Vec<&CharacterDefinition> {
        self.characters
            .get("players")
            .map(|players| players.values().collect())
            .unwrap_or_default()
    }

    /// Get all boss characters
    pub fn get_boss_characters(&self) -> Vec<&CharacterDefinition> {
        self.characters
            .get("bosses")
            .map(|bosses| bosses.values().collect())
            .unwrap_or_default()
    }

    /// Get all enemy characters
    pub fn get_enemy_characters(&self) -> Vec<&CharacterDefinition> {
        self.characters
            .get("enemies")
            .map(|enemies| enemies.values().collect())
            .unwrap_or_default()
    }

    /// Get all characters across all categories
    pub fn get_all_characters(&self) -> Vec<&CharacterDefinition> {
        self.characters
            .values()
            .flat_map(|category| category.values())
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
        // Create players category
        let mut players = HashMap::new();
        players.insert(
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

        // Create bosses category
        let mut bosses = HashMap::new();
        bosses.insert(
            "sunny_dragon".to_string(),
            CharacterDefinition {
                id: "sunny_dragon".to_string(),
                name: "Sunny Dragon".to_string(),
                character_type: CharacterType::Boss,
                animation_config_path:
                    "graphics/characters/bosses/sunny_dragon/sunny_dragon_animations.ron"
                        .to_string(),
                description: "A fearsome dragon".to_string(),
            },
        );

        // Create nested structure
        let mut characters = HashMap::new();
        characters.insert("players".to_string(), players);
        characters.insert("bosses".to_string(), bosses);

        let config = CharactersConfig {
            characters,
            default_player: "fox".to_string(),
        };

        assert!(config.get_character("fox").is_ok());
        assert!(config.get_character("sunny_dragon").is_ok());
        assert!(config.get_default_player().is_ok());
        assert_eq!(config.get_playable_characters().len(), 1);
        assert_eq!(config.get_boss_characters().len(), 1);
        assert_eq!(config.get_all_characters().len(), 2);

        // Validation should pass for unique IDs
        assert!(config.validate_unique_ids().is_ok());
    }

    #[test]
    fn test_duplicate_character_id_detection() {
        // Create players category with "dragon"
        let mut players = HashMap::new();
        players.insert(
            "dragon".to_string(),
            CharacterDefinition {
                id: "dragon".to_string(),
                name: "Player Dragon".to_string(),
                character_type: CharacterType::Player,
                animation_config_path: "graphics/characters/players/dragon/dragon_animations.ron"
                    .to_string(),
                description: "A player dragon".to_string(),
            },
        );

        // Create bosses category with same "dragon" ID
        let mut bosses = HashMap::new();
        bosses.insert(
            "dragon".to_string(),
            CharacterDefinition {
                id: "dragon".to_string(),
                name: "Boss Dragon".to_string(),
                character_type: CharacterType::Boss,
                animation_config_path: "graphics/characters/bosses/dragon/dragon_animations.ron"
                    .to_string(),
                description: "A boss dragon".to_string(),
            },
        );

        // Create nested structure with duplicate ID
        let mut characters = HashMap::new();
        characters.insert("players".to_string(), players);
        characters.insert("bosses".to_string(), bosses);

        let config = CharactersConfig {
            characters,
            default_player: "dragon".to_string(),
        };

        // Validation should fail due to duplicate ID
        let result = config.validate_unique_ids();
        assert!(result.is_err());

        if let Err(CharacterConfigError::DuplicateCharacterId { id, categories }) = result {
            assert_eq!(id, "dragon");
            assert!(categories.contains("players"));
            assert!(categories.contains("bosses"));
        } else {
            panic!("Expected DuplicateCharacterId error");
        }
    }
}
