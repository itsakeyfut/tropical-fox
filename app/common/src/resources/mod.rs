//! Game resources
//!
//! This module will contain all ECS resources used throughout the game.

use bevy::prelude::*;
use std::collections::HashMap;

/// Physics configuration resource
#[derive(Resource, Debug)]
pub struct PhysicsConfig {
    /// Base gravity acceleration (pixels per second squared)
    /// Negative value pulls entities downward
    pub gravity: f32,
    /// Terminal velocity (maximum falling speed)
    pub terminal_velocity: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: -980.0, // Standard gravity in pixels/s²
            terminal_velocity: -500.0,
        }
    }
}

/// Texture atlas data for a single character
#[derive(Debug, Clone)]
pub struct CharacterTextureAtlas {
    /// Handle to the character's sprite texture
    pub texture: Handle<Image>,
    /// Handle to the character's texture atlas layout
    pub layout: Handle<TextureAtlasLayout>,
}

impl CharacterTextureAtlas {
    /// Create new character texture atlas with the given handles
    pub fn new(texture: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self { texture, layout }
    }
}

/// Character sprite assets (texture atlas layouts and textures)
#[derive(Resource, Debug, Default)]
pub struct CharacterAssets {
    /// Map of character ID to texture atlas data
    pub characters: HashMap<String, CharacterTextureAtlas>,
}

impl CharacterAssets {
    /// Create new empty character assets
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
        }
    }

    /// Insert a character's texture atlas data
    pub fn insert(&mut self, character_id: impl Into<String>, atlas: CharacterTextureAtlas) {
        self.characters.insert(character_id.into(), atlas);
    }

    /// Get a character's texture atlas data by ID
    pub fn get(&self, character_id: &str) -> Option<&CharacterTextureAtlas> {
        self.characters.get(character_id)
    }

    /// Get the default character's texture atlas data (fox)
    pub fn get_default(&self) -> Option<&CharacterTextureAtlas> {
        self.characters.get("fox")
    }
}
