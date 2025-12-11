//! Game configuration
//!
//! This module handles loading and managing game configuration.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Physics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSettings {
    /// Base gravity acceleration (pixels per second squared)
    pub gravity: f32,
    /// Terminal velocity (maximum falling speed)
    pub terminal_velocity: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: -980.0,
            terminal_velocity: -500.0,
        }
    }
}

/// Window settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    /// Window width
    pub width: f32,
    /// Window height
    pub height: f32,
    /// Window title
    pub title: String,
    /// Whether the window is resizable
    pub resizable: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1280.0,
            height: 720.0,
            title: "Tropical Fox".to_string(),
            resizable: true,
        }
    }
}

/// Main game settings structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    /// Physics configuration
    pub physics: PhysicsSettings,
    /// Window configuration
    pub window: WindowSettings,
}

/// Configuration loading errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON config: {0}")]
    RonError(#[from] ron::error::SpannedError),
}

/// Load game settings from a RON file
pub fn load_settings<P: AsRef<Path>>(path: P) -> Result<GameSettings, ConfigError> {
    let content = fs::read_to_string(path)?;
    let settings: GameSettings = ron::from_str(&content)?;
    Ok(settings)
}

/// Load game settings from a RON file, or return default if file doesn't exist
pub fn load_settings_or_default<P: AsRef<Path>>(path: P) -> GameSettings {
    match load_settings(path) {
        Ok(settings) => {
            info!("Loaded game settings from file");
            settings
        }
        Err(e) => {
            warn!("Failed to load game settings: {}. Using defaults.", e);
            GameSettings::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = GameSettings::default();
        assert_eq!(settings.physics.gravity, -980.0);
        assert_eq!(settings.window.width, 1280.0);
        assert_eq!(settings.window.height, 720.0);
    }

    #[test]
    fn test_serialize_deserialize() {
        let settings = GameSettings::default();
        let ron_string = ron::to_string(&settings).unwrap();
        let deserialized: GameSettings = ron::from_str(&ron_string).unwrap();

        assert_eq!(deserialized.physics.gravity, settings.physics.gravity);
        assert_eq!(deserialized.window.title, settings.window.title);
    }
}
