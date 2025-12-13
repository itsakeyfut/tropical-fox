//! Error handling for the game
//!
//! # Design Philosophy
//!
//! This game prioritizes graceful degradation over strict error handling:
//! - Errors are logged with context but rarely cause crashes
//! - Fallback values are preferred (default animations, placeholder assets, etc.)
//! - Runtime errors are acceptable if the game can continue functioning
//!
//! # Adding New Error Types
//!
//! 1. Define a new enum with `#[derive(Debug, Error)]`
//! 2. Add it to `GameError` with `#[from]` for automatic conversion
//! 3. Provide helper constructors for common cases
//!
//! # Usage
//!
//! ```rust,ignore
//! // In functions that can fail, return specific error types
//! fn load_sprite(path: &str) -> Result<Sprite, AssetError> {
//!     // ... implementation
//! }
//!
//! // Callers can handle errors or fall back to defaults
//! match load_sprite("player.png") {
//!     Ok(sprite) => sprite,
//!     Err(e) => {
//!         warn!("Failed to load sprite: {}", e);
//!         Sprite::placeholder()
//!     }
//! }
//! ```

use thiserror::Error;

// Import existing Config error types
use crate::config::animation::AnimationConfigError;
use crate::config::characters::CharacterConfigError;
use crate::config::ConfigError;

/// Top-level error type that unifies all subsystem errors
///
/// Primarily used for functions that can fail in multiple ways across different
/// subsystems. For most code, prefer using specific error types like `AnimationError`
/// or `AssetError` for clearer error handling.
///
/// The `#[from]` attribute enables automatic conversion via the `?` operator.
#[derive(Debug, Error)]
pub enum GameError {
    /// Configuration file parsing or I/O errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Animation config file errors (RON parsing, I/O)
    #[error("Animation configuration error: {0}")]
    AnimationConfig(#[from] AnimationConfigError),

    /// Character config file errors (RON parsing, I/O, missing character)
    #[error("Character configuration error: {0}")]
    CharacterConfig(#[from] CharacterConfigError),

    /// Asset loading failures (file not found, invalid format, texture atlas issues)
    #[error("Asset error: {0}")]
    Asset(#[from] AssetError),

    /// Animation clip creation or playback errors (invalid frame ranges, missing clips)
    #[error("Animation error: {0}")]
    Animation(#[from] AnimationError),

    /// Component state errors (missing components, invalid states)
    #[error("Component error: {0}")]
    Component(#[from] ComponentError),

    /// Physics calculation or collision detection errors
    #[error("Physics error: {0}")]
    Physics(#[from] PhysicsError),

    /// Catch-all for unexpected errors that don't fit other categories
    #[error("Unexpected error: {0}")]
    Other(String),
}

/// Errors occurring during asset and resource loading
///
/// These errors typically indicate missing files, corrupted data, or unsupported formats.
/// In most cases, the game should log these errors and fall back to placeholder assets
/// rather than crashing.
#[derive(Debug, Error)]
pub enum AssetError {
    /// The requested asset file does not exist at the specified path
    ///
    /// Common causes:
    /// - Typo in asset path
    /// - Asset not included in build
    /// - Incorrect relative path from assets directory
    #[error("Asset not found: {path}")]
    NotFound { path: String },

    /// Asset file exists but failed to load
    ///
    /// Common causes:
    /// - File corruption
    /// - Insufficient memory
    /// - Permission issues
    /// - Unsupported file format variant
    #[error("Failed to load asset '{path}': {reason}")]
    LoadFailed { path: String, reason: String },

    /// Asset loaded successfully but format is invalid or unexpected
    ///
    /// Common causes:
    /// - Wrong file type (expected .png, got .jpg)
    /// - Corrupted file header
    /// - Unsupported version of format
    #[error("Invalid asset format for '{path}': {reason}")]
    InvalidFormat { path: String, reason: String },

    /// Texture atlas layout or sprite extraction errors
    ///
    /// Common causes:
    /// - Invalid grid dimensions (0 columns/rows)
    /// - Sprite size doesn't evenly divide texture dimensions
    /// - Atlas index out of bounds
    #[error("Texture atlas error: {0}")]
    TextureAtlas(String),
}

impl AssetError {
    /// Construct a "not found" error for a missing asset file
    pub fn not_found(path: impl Into<String>) -> Self {
        AssetError::NotFound { path: path.into() }
    }

    /// Construct a "load failed" error when file exists but loading fails
    pub fn load_failed(path: impl Into<String>, reason: impl std::fmt::Display) -> Self {
        AssetError::LoadFailed {
            path: path.into(),
            reason: reason.to_string(),
        }
    }

    /// Construct an "invalid format" error when file loads but format is wrong
    pub fn invalid_format(path: impl Into<String>, reason: impl Into<String>) -> Self {
        AssetError::InvalidFormat {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Construct a texture atlas error for sprite sheet processing failures
    pub fn texture_atlas(reason: impl Into<String>) -> Self {
        AssetError::TextureAtlas(reason.into())
    }
}

/// Errors related to animation system operations
///
/// These errors occur during animation clip creation, validation, or playback.
/// The game should handle these by falling back to default animations to maintain
/// visual continuity even when animation data is invalid.
#[derive(Debug, Error)]
pub enum AnimationError {
    /// Animation clip name not found in controller's clip map
    ///
    /// This usually indicates a typo in animation name or missing animation definition.
    /// Consider falling back to a default animation like "idle".
    #[error("Animation clip not found: {name}")]
    ClipNotFound { name: String },

    /// General animation clip validation failure
    ///
    /// Use more specific variants (InvalidFrameRange, InvalidFps) when possible.
    #[error("Invalid animation clip: {reason}")]
    InvalidClip { reason: String },

    /// Animation state machine or playback state is inconsistent
    ///
    /// This may indicate a bug in animation controller logic.
    #[error("Animation state error: {0}")]
    StateError(String),

    /// Failed to construct animation from config or other source
    ///
    /// Wraps lower-level errors during animation creation process.
    #[error("Failed to create animation: {0}")]
    CreationFailed(String),

    /// Frame range is invalid (first > last, negative values, etc.)
    ///
    /// Animation clips require `first_frame <= last_frame`.
    /// This validation prevents texture atlas out-of-bounds access.
    #[error("Invalid frame range: first={first}, last={last}. {reason}")]
    InvalidFrameRange {
        first: usize,
        last: usize,
        reason: String,
    },

    /// FPS (frames per second) value is invalid (≤ 0, NaN, infinity)
    ///
    /// FPS must be positive and finite to calculate frame durations.
    #[error("Invalid FPS value: {fps}. {reason}")]
    InvalidFps { fps: f32, reason: String },
}

impl AnimationError {
    /// Construct an error for a missing animation clip
    pub fn clip_not_found(name: impl Into<String>) -> Self {
        AnimationError::ClipNotFound { name: name.into() }
    }

    /// Construct a general invalid clip error
    pub fn invalid_clip(reason: impl Into<String>) -> Self {
        AnimationError::InvalidClip {
            reason: reason.into(),
        }
    }

    /// Construct an error for invalid frame range with detailed context
    pub fn invalid_frame_range(first: usize, last: usize, reason: impl Into<String>) -> Self {
        AnimationError::InvalidFrameRange {
            first,
            last,
            reason: reason.into(),
        }
    }

    /// Construct an error for invalid FPS value with context
    pub fn invalid_fps(fps: f32, reason: impl Into<String>) -> Self {
        AnimationError::InvalidFps {
            fps,
            reason: reason.into(),
        }
    }
}

/// Errors related to Bevy ECS components and entity state
///
/// These errors indicate missing required components or invalid component state.
/// In ECS architectures, missing components often indicate system ordering issues
/// or incomplete entity initialization.
#[derive(Debug, Error)]
pub enum ComponentError {
    /// Required component missing on entity
    ///
    /// This often means:
    /// - Entity not fully initialized (spawned without required components)
    /// - Component removed unexpectedly
    /// - System running before initialization system
    ///
    /// Consider checking system ordering in plugin setup.
    #[error("Required component '{component}' not found{}", entity.as_ref().map(|e| format!(" on entity {}", e)).unwrap_or_default())]
    ComponentNotFound {
        component: String,
        entity: Option<String>,
    },

    /// Component exists but its state is invalid for current operation
    ///
    /// Examples:
    /// - Animation controller with no clips when trying to play
    /// - Velocity with NaN values
    /// - State machine in unexpected state
    #[error("Invalid component state for '{component}': {reason}")]
    InvalidState { component: String, reason: String },

    /// Multiple components' states are mutually inconsistent
    ///
    /// Examples:
    /// - Player marked as grounded but y-velocity is upward
    /// - Animation playing but timer not running
    #[error("Component state inconsistency: {0}")]
    StateInconsistency(String),
}

impl ComponentError {
    /// Construct an error for missing component without entity ID
    pub fn component_not_found(component: impl Into<String>) -> Self {
        ComponentError::ComponentNotFound {
            component: component.into(),
            entity: None,
        }
    }

    /// Construct an error for missing component with specific entity ID for debugging
    pub fn component_not_found_on_entity(
        component: impl Into<String>,
        entity: impl Into<String>,
    ) -> Self {
        ComponentError::ComponentNotFound {
            component: component.into(),
            entity: Some(entity.into()),
        }
    }

    /// Construct an error for invalid component state
    pub fn invalid_state(component: impl Into<String>, reason: impl Into<String>) -> Self {
        ComponentError::InvalidState {
            component: component.into(),
            reason: reason.into(),
        }
    }
}

/// Errors in physics calculations or collision detection
///
/// Physics errors are rare in simple 2D games but can occur with invalid
/// configurations or edge cases in collision math.
#[derive(Debug, Error)]
pub enum PhysicsError {
    /// Physics configuration contains invalid values
    ///
    /// Examples:
    /// - Negative mass or gravity
    /// - NaN or infinity in physics constants
    /// - Invalid collision layer configuration
    #[error("Invalid physics configuration: {0}")]
    InvalidConfig(String),

    /// Collision detection algorithm encountered an error
    ///
    /// This may indicate numerical instability or edge cases in collision math.
    /// Consider using epsilon comparisons for floating-point collision checks.
    #[error("Collision detection error: {0}")]
    CollisionError(String),
}

impl PhysicsError {
    /// Construct an error for invalid physics configuration
    pub fn invalid_config(reason: impl Into<String>) -> Self {
        PhysicsError::InvalidConfig(reason.into())
    }

    /// Construct an error for collision detection failures
    pub fn collision_error(reason: impl Into<String>) -> Self {
        PhysicsError::CollisionError(reason.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_error_creation() {
        let err = AnimationError::invalid_frame_range(10, 5, "first > last");
        assert!(err.to_string().contains("first=10"));
        assert!(err.to_string().contains("last=5"));
    }

    #[test]
    fn test_asset_error_helpers() {
        let err = AssetError::not_found("path/to/asset.png");
        assert!(err.to_string().contains("path/to/asset.png"));

        let err = AssetError::load_failed("texture.png", "File not found");
        assert!(err.to_string().contains("texture.png"));
        assert!(err.to_string().contains("File not found"));
    }

    #[test]
    fn test_component_error_helpers() {
        let err = ComponentError::component_not_found("Transform");
        assert!(err.to_string().contains("Transform"));

        let err = ComponentError::component_not_found_on_entity("Velocity", "Entity(123)");
        assert!(err.to_string().contains("Velocity"));
        assert!(err.to_string().contains("Entity(123)"));
    }

    #[test]
    fn test_game_error_from_conversion() {
        let anim_err = AnimationError::clip_not_found("idle");
        let game_err: GameError = anim_err.into();
        assert!(game_err.to_string().contains("Animation error"));
        assert!(game_err.to_string().contains("idle"));
    }
}
