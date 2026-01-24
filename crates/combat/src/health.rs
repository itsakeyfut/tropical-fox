//! Health system components and utilities
//!
//! Provides health tracking for both players and enemies.

use bevy::prelude::*;

/// Health component for entities that can take damage
#[derive(Component, Debug)]
pub struct Health {
    /// Current health points
    pub current: f32,
    /// Maximum health points
    pub max: f32,
}

impl Health {
    /// Create a new health component with full health
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    /// Create a new health component with specified current and max values
    #[allow(dead_code)]
    pub fn with_current(current: f32, max: f32) -> Self {
        Self {
            current: current.min(max).max(0.0),
            max,
        }
    }

    /// Apply damage to this entity
    ///
    /// Returns true if the entity died (health reached 0)
    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.current = (self.current - amount).max(0.0);
        self.current <= 0.0
    }

    /// Heal this entity
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Check if the entity is still alive
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    /// Get health as a ratio (0.0 to 1.0)
    pub fn ratio(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100.0)
    }
}

/// Marker component for player health (used for filtering)
#[derive(Component, Debug, Default)]
pub struct PlayerHealth;

/// Marker component for enemy health (used for filtering)
#[derive(Component, Debug, Default)]
pub struct EnemyHealth;

/// Component for tracking lives (for respawn system)
#[derive(Component, Debug)]
pub struct Lives {
    /// Current number of lives remaining
    pub current: u32,
    /// Maximum number of lives
    pub max: u32,
}

impl Lives {
    /// Create a new lives component
    pub fn new(lives: u32) -> Self {
        Self {
            current: lives,
            max: lives,
        }
    }

    /// Lose a life, returns true if no lives remain
    pub fn lose_life(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
        }
        self.current == 0
    }

    /// Gain a life (capped at max)
    #[allow(dead_code)]
    pub fn gain_life(&mut self) {
        self.current = (self.current + 1).min(self.max);
    }

    /// Check if player has lives remaining
    #[allow(dead_code)]
    pub fn has_lives(&self) -> bool {
        self.current > 0
    }
}

impl Default for Lives {
    fn default() -> Self {
        Self::new(3)
    }
}

/// Resource for tracking player spawn point (for respawn)
#[derive(Resource, Debug)]
pub struct PlayerSpawnPoint {
    /// Position to respawn the player
    pub position: Vec3,
}

impl Default for PlayerSpawnPoint {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 100.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_creation() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
        assert!(health.is_alive());
    }

    #[test]
    fn test_health_damage() {
        let mut health = Health::new(100.0);

        let died = health.take_damage(30.0);
        assert!(!died);
        assert_eq!(health.current, 70.0);

        let died = health.take_damage(100.0);
        assert!(died);
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(100.0);
        health.take_damage(50.0);

        health.heal(30.0);
        assert_eq!(health.current, 80.0);

        // Heal beyond max should cap at max
        health.heal(100.0);
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_health_ratio() {
        let mut health = Health::new(100.0);
        assert!((health.ratio() - 1.0).abs() < f32::EPSILON);

        health.take_damage(50.0);
        assert!((health.ratio() - 0.5).abs() < f32::EPSILON);

        health.take_damage(50.0);
        assert!((health.ratio() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lives() {
        let mut lives = Lives::new(3);
        assert_eq!(lives.current, 3);

        let game_over = lives.lose_life();
        assert!(!game_over);
        assert_eq!(lives.current, 2);

        lives.lose_life();
        let game_over = lives.lose_life();
        assert!(game_over);
        assert_eq!(lives.current, 0);
    }
}
