//! Enemy configuration and management
//!
//! Handles enemy type definitions and loading from RON files.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Enemy AI behavior type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AIBehaviorType {
    /// Enemy stays idle
    #[default]
    Idle,
    /// Enemy patrols between points
    Patrol,
    /// Enemy chases player when in range
    Chase,
    /// Enemy flies in patterns
    Flying,
}

/// Flying pattern configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlyingPatternConfig {
    /// Sine wave movement
    SineWave {
        amplitude: f32,
        frequency: f32,
        speed: f32,
    },
    /// Circular movement
    Circle { radius: f32, speed: f32 },
    /// Hovering in place
    Hover { range: f32, speed: f32 },
    /// Figure-8 pattern
    Figure8 { width: f32, height: f32, speed: f32 },
}

impl Default for FlyingPatternConfig {
    fn default() -> Self {
        Self::Hover {
            range: 20.0,
            speed: 2.0,
        }
    }
}

/// AI configuration for an enemy type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Primary AI behavior type
    pub behavior: AIBehaviorType,
    /// Detection range for chase AI (pixels)
    #[serde(default = "default_detection_range")]
    pub detection_range: f32,
    /// Attack range for chase AI (pixels)
    #[serde(default = "default_attack_range")]
    pub attack_range: f32,
    /// Patrol distance for patrol AI (pixels)
    #[serde(default = "default_patrol_distance")]
    pub patrol_distance: f32,
    /// Wait time at patrol points (seconds)
    #[serde(default = "default_wait_time")]
    pub wait_time: f32,
    /// Flying pattern configuration
    #[serde(default)]
    pub flying_pattern: Option<FlyingPatternConfig>,
}

fn default_detection_range() -> f32 {
    200.0
}
fn default_attack_range() -> f32 {
    30.0
}
fn default_patrol_distance() -> f32 {
    100.0
}
fn default_wait_time() -> f32 {
    1.0
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            behavior: AIBehaviorType::Idle,
            detection_range: 200.0,
            attack_range: 30.0,
            patrol_distance: 100.0,
            wait_time: 1.0,
            flying_pattern: None,
        }
    }
}

/// Statistics configuration for an enemy type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyStatsConfig {
    /// Maximum health
    #[serde(default = "default_health")]
    pub health: f32,
    /// Movement speed (pixels per second)
    #[serde(default = "default_move_speed")]
    pub move_speed: f32,
    /// Contact damage dealt to player
    #[serde(default = "default_damage")]
    pub damage: f32,
    /// Knockback force on hit
    #[serde(default = "default_knockback")]
    pub knockback_force: f32,
    /// Score value when defeated
    #[serde(default = "default_score")]
    pub score_value: u32,
}

fn default_health() -> f32 {
    30.0
}
fn default_move_speed() -> f32 {
    60.0
}
fn default_damage() -> f32 {
    10.0
}
fn default_knockback() -> f32 {
    150.0
}
fn default_score() -> u32 {
    100
}

impl Default for EnemyStatsConfig {
    fn default() -> Self {
        Self {
            health: 30.0,
            move_speed: 60.0,
            damage: 10.0,
            knockback_force: 150.0,
            score_value: 100,
        }
    }
}

/// Collider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColliderConfig {
    /// Collider size (width, height)
    pub size: (f32, f32),
    /// Collider offset from center
    #[serde(default)]
    pub offset: (f32, f32),
}

impl Default for ColliderConfig {
    fn default() -> Self {
        Self {
            size: (32.0, 32.0),
            offset: (0.0, 0.0),
        }
    }
}

/// Projectile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileConfig {
    /// Whether this enemy shoots projectiles
    #[serde(default)]
    pub enabled: bool,
    /// Projectile damage
    #[serde(default = "default_projectile_damage")]
    pub damage: f32,
    /// Projectile speed (pixels per second)
    #[serde(default = "default_projectile_speed")]
    pub speed: f32,
    /// Fire rate (seconds between shots)
    #[serde(default = "default_fire_rate")]
    pub fire_rate: f32,
    /// Range to start shooting
    #[serde(default = "default_shoot_range")]
    pub range: f32,
}

fn default_projectile_damage() -> f32 {
    5.0
}
fn default_projectile_speed() -> f32 {
    200.0
}
fn default_fire_rate() -> f32 {
    2.0
}
fn default_shoot_range() -> f32 {
    300.0
}

impl Default for ProjectileConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            damage: 5.0,
            speed: 200.0,
            fire_rate: 2.0,
            range: 300.0,
        }
    }
}

/// Drop item configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropConfig {
    /// Chance to drop an item (0.0 to 1.0)
    #[serde(default = "default_drop_chance")]
    pub drop_chance: f32,
    /// List of possible drop item types
    #[serde(default = "default_drop_items")]
    pub items: Vec<String>,
}

fn default_drop_chance() -> f32 {
    0.3
}

fn default_drop_items() -> Vec<String> {
    vec!["coin".to_string()]
}

impl Default for DropConfig {
    fn default() -> Self {
        Self {
            drop_chance: default_drop_chance(),
            items: default_drop_items(),
        }
    }
}

/// Complete enemy type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyTypeConfig {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    #[serde(default)]
    pub description: String,
    /// Enemy statistics
    #[serde(default)]
    pub stats: EnemyStatsConfig,
    /// AI configuration
    #[serde(default)]
    pub ai: AIConfig,
    /// Collider configuration
    #[serde(default)]
    pub collider: ColliderConfig,
    /// Projectile configuration
    #[serde(default)]
    pub projectile: ProjectileConfig,
    /// Drop configuration
    #[serde(default)]
    pub drop: DropConfig,
    /// Whether enemy is affected by gravity
    #[serde(default = "default_gravity")]
    pub has_gravity: bool,
}

fn default_gravity() -> bool {
    true
}

impl Default for EnemyTypeConfig {
    fn default() -> Self {
        Self {
            id: "unknown".to_string(),
            name: "Unknown Enemy".to_string(),
            description: String::new(),
            stats: EnemyStatsConfig::default(),
            ai: AIConfig::default(),
            collider: ColliderConfig::default(),
            projectile: ProjectileConfig::default(),
            drop: DropConfig::default(),
            has_gravity: true,
        }
    }
}

/// Enemy configuration file format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnemiesConfig {
    /// Map of enemy type ID to configuration
    pub enemies: HashMap<String, EnemyTypeConfig>,
}

impl EnemiesConfig {
    /// Get an enemy type configuration by ID
    pub fn get(&self, id: &str) -> Option<&EnemyTypeConfig> {
        self.enemies.get(id)
    }

    /// Get all enemy type IDs
    pub fn enemy_ids(&self) -> impl Iterator<Item = &str> {
        self.enemies.keys().map(|s| s.as_str())
    }

    /// Get all enemy type configurations
    pub fn all(&self) -> impl Iterator<Item = &EnemyTypeConfig> {
        self.enemies.values()
    }
}

/// Enemy configuration loading errors
#[derive(Debug, Error)]
pub enum EnemyConfigError {
    #[error("Failed to read enemy config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse RON enemy config: {0}")]
    RonError(#[from] ron::error::SpannedError),

    #[error("Enemy type not found: {0}")]
    EnemyNotFound(String),
}

/// Load enemies configuration from a RON file
pub fn load_enemies_config<P: AsRef<Path>>(path: P) -> Result<EnemiesConfig, EnemyConfigError> {
    let content = fs::read_to_string(path)?;
    let config: EnemiesConfig = ron::from_str(&content)?;
    Ok(config)
}

/// Load enemies configuration from a RON file, or return default if loading/parsing fails
pub fn load_enemies_config_optional<P: AsRef<Path>>(path: P) -> EnemiesConfig {
    match load_enemies_config(path) {
        Ok(config) => {
            info!("Loaded enemies config with {} types", config.enemies.len());
            config
        }
        Err(e) => {
            warn!("Failed to load enemies config: {}. Using default.", e);
            EnemiesConfig::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_stats_config_default() {
        let stats = EnemyStatsConfig::default();
        assert_eq!(stats.health, 30.0);
        assert_eq!(stats.move_speed, 60.0);
        assert_eq!(stats.damage, 10.0);
    }

    #[test]
    fn test_ai_config_default() {
        let ai = AIConfig::default();
        assert_eq!(ai.behavior, AIBehaviorType::Idle);
        assert_eq!(ai.detection_range, 200.0);
    }

    #[test]
    fn test_enemies_config() {
        let mut enemies = HashMap::new();
        enemies.insert(
            "slime".to_string(),
            EnemyTypeConfig {
                id: "slime".to_string(),
                name: "Slime".to_string(),
                ..Default::default()
            },
        );

        let config = EnemiesConfig { enemies };
        assert!(config.get("slime").is_some());
        assert!(config.get("unknown").is_none());
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut enemies = HashMap::new();
        enemies.insert(
            "slime".to_string(),
            EnemyTypeConfig {
                id: "slime".to_string(),
                name: "Slime".to_string(),
                ..Default::default()
            },
        );

        let config = EnemiesConfig { enemies };
        let ron_string = ron::to_string(&config).unwrap();
        let deserialized: EnemiesConfig = ron::from_str(&ron_string).unwrap();

        assert!(deserialized.get("slime").is_some());
    }
}
