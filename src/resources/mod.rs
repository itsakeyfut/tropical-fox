//! Game resources
//!
//! This module will contain all ECS resources used throughout the game.

use bevy::prelude::*;

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

/// Character sprite assets (texture atlas layouts and textures)
#[derive(Resource, Debug)]
pub struct CharacterAssets {
    /// Handle to the fox character's texture atlas layout
    pub fox_layout: Handle<TextureAtlasLayout>,
    /// Handle to the fox character's sprite texture
    pub fox_texture: Handle<Image>,
}

impl CharacterAssets {
    /// Create new character assets with the given handles
    pub fn new(fox_layout: Handle<TextureAtlasLayout>, fox_texture: Handle<Image>) -> Self {
        Self {
            fox_layout,
            fox_texture,
        }
    }
}
