//! Animation configuration structures and loading
//!
//! Defines the format for animation configuration files (RON)

use crate::components::{AnimationClip, AnimationController};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// RON configuration for a complete animation setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Path to the sprite sheet texture (relative to assets directory)
    pub spritesheet_path: String,
    /// Size of each sprite in pixels (width, height)
    pub sprite_size: (u32, u32),
    /// Number of columns in the sprite sheet
    pub columns: u32,
    /// Number of rows in the sprite sheet
    pub rows: u32,
    /// Map of animation name to animation clip configuration
    pub clips: HashMap<String, AnimationClipConfig>,
    /// Name of the default animation to play on start
    pub default_animation: String,
}

/// RON configuration for a single animation clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClipConfig {
    /// Index of the first frame in the sprite sheet (0-based)
    pub first: usize,
    /// Index of the last frame in the sprite sheet (0-based)
    pub last: usize,
    /// Frames per second (playback speed)
    pub fps: f32,
}

impl TryFrom<AnimationClipConfig> for AnimationClip {
    type Error = String;

    fn try_from(config: AnimationClipConfig) -> Result<Self, Self::Error> {
        AnimationClip::new(config.first, config.last, config.fps)
    }
}

impl TryFrom<AnimationConfig> for AnimationController {
    type Error = String;

    fn try_from(config: AnimationConfig) -> Result<Self, Self::Error> {
        let mut controller = AnimationController::new();

        for (name, clip_config) in config.clips {
            let clip = AnimationClip::try_from(clip_config).map_err(|e| {
                format!("Failed to create animation clip '{}': {}", name, e)
            })?;
            controller.add_animation(name, clip);
        }

        controller.current_animation = config.default_animation.clone();

        Ok(controller)
    }
}

/// Animation configuration loading errors
#[derive(Debug, Error)]
pub enum AnimationConfigError {
    #[error("Failed to read animation config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON animation config: {0}")]
    RonError(#[from] ron::error::SpannedError),
}

/// Load animation configuration from a RON file
pub fn load_animation_config<P: AsRef<Path>>(
    path: P,
) -> Result<AnimationConfig, AnimationConfigError> {
    let content = fs::read_to_string(path)?;
    let config: AnimationConfig = ron::from_str(&content)?;
    Ok(config)
}

/// Load animation configuration from a RON file, or return None if file doesn't exist
pub fn load_animation_config_optional<P: AsRef<Path>>(path: P) -> Option<AnimationConfig> {
    match load_animation_config(path) {
        Ok(config) => Some(config),
        Err(e) => {
            warn!("Failed to load animation config: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_clip_config_conversion() {
        let config = AnimationClipConfig {
            first: 0,
            last: 5,
            fps: 10.0,
        };

        let clip: AnimationClip = config.try_into().unwrap();
        assert_eq!(clip.first_frame, 0);
        assert_eq!(clip.last_frame, 5);
        assert_eq!(clip.fps, 10.0);
    }

    #[test]
    fn test_animation_config_conversion() {
        let mut clips = HashMap::new();
        clips.insert(
            "idle".to_string(),
            AnimationClipConfig {
                first: 0,
                last: 3,
                fps: 8.0,
            },
        );
        clips.insert(
            "run".to_string(),
            AnimationClipConfig {
                first: 4,
                last: 9,
                fps: 12.0,
            },
        );

        let config = AnimationConfig {
            spritesheet_path: "graphics/player.png".to_string(),
            sprite_size: (32, 32),
            columns: 10,
            rows: 10,
            clips,
            default_animation: "idle".to_string(),
        };

        let controller: AnimationController = config.try_into().unwrap();
        assert_eq!(controller.current_animation, "idle");
        assert_eq!(controller.animations.len(), 2);
        assert!(controller.animations.contains_key("idle"));
        assert!(controller.animations.contains_key("run"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut clips = HashMap::new();
        clips.insert(
            "idle".to_string(),
            AnimationClipConfig {
                first: 0,
                last: 3,
                fps: 8.0,
            },
        );

        let config = AnimationConfig {
            spritesheet_path: "graphics/player.png".to_string(),
            sprite_size: (32, 32),
            columns: 10,
            rows: 5,
            clips,
            default_animation: "idle".to_string(),
        };

        let ron_string = ron::to_string(&config).unwrap();
        let deserialized: AnimationConfig = ron::from_str(&ron_string).unwrap();

        assert_eq!(deserialized.spritesheet_path, config.spritesheet_path);
        assert_eq!(deserialized.sprite_size, config.sprite_size);
        assert_eq!(deserialized.default_animation, config.default_animation);
    }
}
