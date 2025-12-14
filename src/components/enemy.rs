//! Enemy components
//!
//! This module contains all ECS components used for enemy entities.

use bevy::prelude::*;

/// Marker component for enemy entities
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    /// Enemy type identifier (e.g., "slime", "bat", "goblin")
    pub enemy_type: String,
    /// True if enemy is facing right, false if facing left
    pub facing_right: bool,
    /// True if the sprite faces left by default (requires flip_x inversion)
    pub sprite_faces_left: bool,
}

impl Enemy {
    /// Create a new enemy with the specified type
    pub fn new(enemy_type: impl Into<String>) -> Self {
        let enemy_type = enemy_type.into();
        // Some sprites face left by default
        let sprite_faces_left = matches!(enemy_type.as_str(), "ant");
        Self {
            enemy_type,
            facing_right: false,
            sprite_faces_left,
        }
    }
}

/// Enemy statistics component
#[derive(Component, Debug, Clone)]
pub struct EnemyStats {
    /// Movement speed (pixels per second)
    pub move_speed: f32,
    /// Contact damage dealt to player
    pub damage: f32,
    /// Knockback force applied to player on hit
    pub knockback_force: f32,
    /// Score value when defeated
    pub score_value: u32,
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            move_speed: 60.0,
            damage: 10.0,
            knockback_force: 150.0,
            score_value: 100,
        }
    }
}

/// AI behavior type for enemies
#[derive(Component, Debug, Clone, Default)]
pub enum EnemyAI {
    /// No AI - enemy stays still
    #[default]
    Idle,
    /// Patrol between waypoints
    Patrol(PatrolAI),
    /// Chase player when in range
    Chase(ChaseAI),
    /// Flying movement patterns
    Flying(FlyingAI),
}

/// Patrol AI component - enemy moves between waypoints
#[derive(Debug, Clone)]
pub struct PatrolAI {
    /// List of patrol points (local offsets from spawn position)
    pub patrol_points: Vec<Vec2>,
    /// Current target point index
    pub current_point_index: usize,
    /// Time to wait at each waypoint (seconds)
    pub wait_time: f32,
    /// Current wait timer
    pub wait_timer: f32,
    /// Original spawn position (patrol points are relative to this)
    pub origin: Vec2,
}

impl PatrolAI {
    /// Create a new patrol AI with the given waypoints
    pub fn new(patrol_points: Vec<Vec2>, wait_time: f32, origin: Vec2) -> Self {
        Self {
            patrol_points,
            current_point_index: 0,
            wait_time,
            wait_timer: 0.0,
            origin,
        }
    }

    /// Create a simple left-right patrol pattern
    pub fn horizontal(distance: f32, wait_time: f32, origin: Vec2) -> Self {
        Self::new(
            vec![Vec2::new(-distance, 0.0), Vec2::new(distance, 0.0)],
            wait_time,
            origin,
        )
    }

    /// Get the current target position in world coordinates
    pub fn current_target(&self) -> Vec2 {
        self.origin + self.patrol_points[self.current_point_index]
    }

    /// Advance to the next patrol point
    pub fn advance(&mut self) {
        self.current_point_index = (self.current_point_index + 1) % self.patrol_points.len();
        self.wait_timer = 0.0;
    }
}

/// Chase AI component - enemy chases player when in detection range
#[derive(Debug, Clone)]
pub struct ChaseAI {
    /// Detection range to start chasing (pixels)
    pub detection_range: f32,
    /// Range to stop chasing and attack (pixels)
    pub attack_range: f32,
    /// Whether currently chasing the player
    pub is_chasing: bool,
    /// Time since last seeing player (for losing aggro)
    pub lost_sight_timer: f32,
    /// Time to wait before losing aggro (seconds)
    pub lose_aggro_time: f32,
}

impl ChaseAI {
    /// Create a new chase AI with the given ranges
    pub fn new(detection_range: f32, attack_range: f32) -> Self {
        Self {
            detection_range,
            attack_range,
            is_chasing: false,
            lost_sight_timer: 0.0,
            lose_aggro_time: 2.0,
        }
    }
}

impl Default for ChaseAI {
    fn default() -> Self {
        Self::new(200.0, 30.0)
    }
}

/// Flying AI component - enemy follows movement patterns in air
#[derive(Debug, Clone)]
pub struct FlyingAI {
    /// Movement pattern type
    pub pattern: FlyingPattern,
    /// Elapsed time for pattern calculation
    pub time: f32,
    /// Origin position for pattern
    pub origin: Vec2,
}

impl FlyingAI {
    /// Create a new flying AI with the given pattern
    pub fn new(pattern: FlyingPattern, origin: Vec2) -> Self {
        Self {
            pattern,
            time: 0.0,
            origin,
        }
    }
}

/// Flying movement patterns
#[derive(Debug, Clone)]
pub enum FlyingPattern {
    /// Sine wave horizontal movement
    SineWave {
        /// Vertical amplitude of the wave
        amplitude: f32,
        /// Horizontal frequency
        frequency: f32,
        /// Horizontal speed (pixels per second)
        speed: f32,
    },
    /// Circular movement around origin
    Circle {
        /// Radius of the circle
        radius: f32,
        /// Angular speed (radians per second)
        speed: f32,
    },
    /// Vertical hovering in place
    Hover {
        /// Vertical range of movement
        range: f32,
        /// Oscillation speed
        speed: f32,
    },
    /// Figure-8 pattern
    Figure8 {
        /// Width of the figure-8
        width: f32,
        /// Height of the figure-8
        height: f32,
        /// Speed multiplier
        speed: f32,
    },
}

impl Default for FlyingPattern {
    fn default() -> Self {
        Self::Hover {
            range: 20.0,
            speed: 2.0,
        }
    }
}

/// Contact damage component - deals damage on collision with player
#[derive(Component, Debug, Clone)]
pub struct ContactDamage {
    /// Damage dealt on contact
    pub damage: f32,
    /// Knockback force applied to target
    pub knockback_force: f32,
    /// Cooldown between damage ticks (prevents rapid damage)
    pub cooldown: f32,
    /// Current cooldown timer
    pub cooldown_timer: f32,
}

impl ContactDamage {
    /// Create a new contact damage component
    pub fn new(damage: f32, knockback_force: f32, cooldown: f32) -> Self {
        Self {
            damage,
            knockback_force,
            cooldown,
            cooldown_timer: 0.0,
        }
    }

    /// Check if contact damage can be applied
    pub fn can_damage(&self) -> bool {
        self.cooldown_timer <= 0.0
    }

    /// Reset the cooldown timer
    pub fn reset_cooldown(&mut self) {
        self.cooldown_timer = self.cooldown;
    }

    /// Update the cooldown timer
    pub fn tick(&mut self, delta: f32) {
        if self.cooldown_timer > 0.0 {
            self.cooldown_timer -= delta;
        }
    }
}

impl Default for ContactDamage {
    fn default() -> Self {
        Self::new(10.0, 150.0, 0.5)
    }
}

/// Projectile shooter component - enemy can fire projectiles
#[derive(Component, Debug, Clone)]
pub struct ProjectileShooter {
    /// Damage per projectile
    pub damage: f32,
    /// Projectile speed (pixels per second)
    pub projectile_speed: f32,
    /// Time between shots (seconds)
    pub fire_rate: f32,
    /// Current fire cooldown timer
    pub fire_timer: f32,
    /// Range at which to start shooting
    pub range: f32,
}

impl ProjectileShooter {
    /// Create a new projectile shooter
    pub fn new(damage: f32, projectile_speed: f32, fire_rate: f32, range: f32) -> Self {
        Self {
            damage,
            projectile_speed,
            fire_rate,
            fire_timer: 0.0,
            range,
        }
    }

    /// Check if ready to fire
    pub fn can_fire(&self) -> bool {
        self.fire_timer <= 0.0
    }

    /// Reset fire timer after shooting
    pub fn reset_timer(&mut self) {
        self.fire_timer = self.fire_rate;
    }

    /// Update the fire timer
    pub fn tick(&mut self, delta: f32) {
        if self.fire_timer > 0.0 {
            self.fire_timer -= delta;
        }
    }
}

impl Default for ProjectileShooter {
    fn default() -> Self {
        Self::new(5.0, 200.0, 2.0, 300.0)
    }
}

/// Enemy projectile component
#[derive(Component, Debug)]
pub struct EnemyProjectile {
    /// Damage dealt on hit
    pub damage: f32,
    /// Knockback force
    pub knockback: f32,
    /// Velocity of the projectile
    pub velocity: Vec2,
    /// Lifetime timer
    pub lifetime: Timer,
}

impl EnemyProjectile {
    /// Create a new enemy projectile
    pub fn new(damage: f32, knockback: f32, velocity: Vec2, lifetime_secs: f32) -> Self {
        Self {
            damage,
            knockback,
            velocity,
            lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy::new("slime");
        assert_eq!(enemy.enemy_type, "slime");
        assert!(!enemy.facing_right);
        assert!(!enemy.sprite_faces_left);

        // Ant sprite faces left by default
        let ant = Enemy::new("ant");
        assert!(ant.sprite_faces_left);
    }

    #[test]
    fn test_patrol_ai() {
        let mut patrol = PatrolAI::horizontal(100.0, 0.5, Vec2::ZERO);
        assert_eq!(patrol.patrol_points.len(), 2);
        assert_eq!(patrol.current_target(), Vec2::new(-100.0, 0.0));

        patrol.advance();
        assert_eq!(patrol.current_target(), Vec2::new(100.0, 0.0));

        patrol.advance();
        assert_eq!(patrol.current_target(), Vec2::new(-100.0, 0.0));
    }

    #[test]
    fn test_contact_damage() {
        let mut contact = ContactDamage::new(10.0, 100.0, 0.5);
        assert!(contact.can_damage());

        contact.reset_cooldown();
        assert!(!contact.can_damage());

        contact.tick(0.6);
        assert!(contact.can_damage());
    }

    #[test]
    fn test_projectile_shooter() {
        let mut shooter = ProjectileShooter::new(5.0, 200.0, 1.0, 300.0);
        assert!(shooter.can_fire());

        shooter.reset_timer();
        assert!(!shooter.can_fire());

        shooter.tick(1.1);
        assert!(shooter.can_fire());
    }
}
