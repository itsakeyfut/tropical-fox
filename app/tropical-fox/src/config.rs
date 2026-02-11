//! Game settings configuration for Tropical Fox binary
//!
//! This module handles loading and managing the main game settings.

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
    pub acceleration: f32,
    /// Horizontal deceleration rate when on ground (higher = more responsive)
    pub deceleration: f32,
    /// Horizontal deceleration rate when in air (air friction)
    pub air_deceleration: f32,
    /// Jump force (upward velocity in pixels per second)
    pub jump_force: f32,
    /// Coyote time duration (seconds)
    pub coyote_time: f32,
    /// Jump buffer duration (seconds)
    pub jump_buffer_time: f32,
    /// Jump cut multiplier
    pub jump_cut_multiplier: f32,
    /// Dash speed (pixels per second)
    pub dash_speed: f32,
    /// Dash duration (seconds)
    pub dash_duration: f32,
    /// Maximum number of air dashes
    pub max_air_dashes: u32,
    /// Wall jump horizontal force
    pub wall_jump_force_x: f32,
    /// Wall jump vertical force
    pub wall_jump_force_y: f32,
    /// Wall slide speed
    pub wall_slide_speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            acceleration: 10.0,
            deceleration: 15.0,
            air_deceleration: 20.0,
            jump_force: 400.0,
            coyote_time: 0.15,
            jump_buffer_time: 0.15,
            jump_cut_multiplier: 0.4,
            dash_speed: 500.0,
            dash_duration: 0.15,
            max_air_dashes: 1,
            wall_jump_force_x: 300.0,
            wall_jump_force_y: 400.0,
            wall_slide_speed: -100.0,
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, Resource, Asset, TypePath)]
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
