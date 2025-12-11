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

/// Player movement settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSettings {
    /// Maximum horizontal movement speed (pixels per second)
    pub move_speed: f32,
    /// Horizontal acceleration rate (higher = more responsive)
    /// Multiplied by delta time to create frame-rate independent interpolation
    pub acceleration: f32,
    /// Horizontal deceleration rate when on ground (higher = more responsive)
    pub deceleration: f32,
    /// Horizontal deceleration rate when in air (air friction)
    /// Higher values = more responsive air control and better landing precision
    /// Celeste uses strong air friction for precise platforming
    pub air_deceleration: f32,
    /// Jump force (upward velocity in pixels per second)
    pub jump_force: f32,
    /// Coyote time duration (seconds) - time after leaving ground where jump is still allowed
    pub coyote_time: f32,
    /// Jump buffer duration (seconds) - time before landing where jump input is remembered
    pub jump_buffer_time: f32,
    /// Jump cut multiplier - reduces upward velocity when jump button is released early
    /// Range: 0.0-1.0 (lower = more dramatic cut)
    pub jump_cut_multiplier: f32,
    /// Dash speed (pixels per second)
    pub dash_speed: f32,
    /// Dash duration (seconds)
    pub dash_duration: f32,
    /// Maximum number of air dashes before needing to touch ground
    pub max_air_dashes: u32,
    /// Wall jump horizontal force (pixels per second)
    pub wall_jump_force_x: f32,
    /// Wall jump vertical force (pixels per second)
    pub wall_jump_force_y: f32,
    /// Wall slide speed (pixels per second, negative = downward)
    pub wall_slide_speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            acceleration: 10.0,
            deceleration: 15.0,
            air_deceleration: 20.0, // Stronger than ground deceleration for better air control
            jump_force: 400.0,
            coyote_time: 0.15,
            jump_buffer_time: 0.15,
            jump_cut_multiplier: 0.4,
            dash_speed: 500.0,        // Fast burst of speed
            dash_duration: 0.15,      // Short duration like Celeste
            max_air_dashes: 1,        // One air dash like Celeste
            wall_jump_force_x: 300.0, // Horizontal push away from wall
            wall_jump_force_y: 400.0, // Vertical jump force (same as normal jump)
            wall_slide_speed: -100.0, // Slow slide down wall
        }
    }
}

/// Window settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Window title
    pub title: String,
    /// Whether the window is resizable
    pub resizable: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "Tropical Fox".to_string(),
            resizable: true,
        }
    }
}

/// Main game settings structure
#[derive(Debug, Clone, Serialize, Deserialize, Default, Resource)]
pub struct GameSettings {
    /// Physics configuration
    pub physics: PhysicsSettings,
    /// Window configuration
    pub window: WindowSettings,
    /// Player configuration
    pub player: PlayerSettings,
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
        assert_eq!(settings.window.width, 1280);
        assert_eq!(settings.window.height, 720);
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
