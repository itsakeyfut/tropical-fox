//! Attack system components and systems
//!
//! Handles attack hitbox creation, collision detection, and multi-hit prevention.

use bevy::prelude::*;
use std::collections::HashSet;

use super::{DamageEvent, EnemyHealth, PlayerHealth};
use crate::components::{Collider, Player};

/// Attack hitbox component
#[derive(Component, Debug)]
pub struct Attack {
    /// Damage dealt on hit
    pub damage: f32,
    /// Knockback force on hit
    pub knockback: f32,
    /// Lifetime timer (attack despawns when finished)
    pub lifetime: Timer,
    /// Entities already hit by this attack (prevents multi-hit)
    pub hit_entities: HashSet<Entity>,
}

impl Attack {
    /// Create a new attack with specified damage, knockback, and duration
    pub fn new(damage: f32, knockback: f32, duration_secs: f32) -> Self {
        Self {
            damage,
            knockback,
            lifetime: Timer::from_seconds(duration_secs, TimerMode::Once),
            hit_entities: HashSet::new(),
        }
    }
}

/// Attack type variants
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    /// Basic attack: moderate damage and range
    Basic,
    /// Charged attack: high damage and range
    Charged,
    /// Aerial attack: downward attack in air
    Aerial,
}

/// Attack cooldown component (prevents attack spam)
#[derive(Component, Debug)]
pub struct AttackCooldown {
    /// Cooldown timer
    pub timer: Timer,
}

impl AttackCooldown {
    /// Create a new cooldown with specified duration
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }

    /// Check if attack is ready (cooldown finished)
    pub fn can_attack(&self) -> bool {
        self.timer.is_finished()
    }

    /// Reset the cooldown timer
    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

impl Default for AttackCooldown {
    fn default() -> Self {
        Self::new(0.3) // 300ms cooldown by default
    }
}

/// Marker component indicating an entity is currently attacking
#[derive(Component, Debug, Default)]
pub struct Attacking;

/// Marker for player-owned attacks
#[derive(Component, Debug, Default)]
pub struct PlayerAttack;

/// Marker for enemy-owned attacks
#[derive(Component, Debug, Default)]
pub struct EnemyAttack;

/// Hurtbox component - marks an entity as damageable
#[derive(Component, Debug, Default)]
pub struct Hurtbox;

/// Handle player attack input
pub fn player_attack_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut AttackCooldown, &Player), With<Player>>,
) {
    let Ok((player_entity, transform, mut cooldown, player)) = player_query.single_mut() else {
        return;
    };

    // X key or J key for basic attack
    if (keyboard.just_pressed(KeyCode::KeyX) || keyboard.just_pressed(KeyCode::KeyJ))
        && cooldown.can_attack()
    {
        cooldown.reset();

        // Mark player as attacking
        commands.entity(player_entity).insert(Attacking);

        // Determine attack direction based on facing
        let facing_dir = if player.facing_right { 1.0 } else { -1.0 };

        // Spawn attack hitbox in front of player
        let attack_offset = Vec3::new(40.0 * facing_dir, 0.0, 0.0);
        let attack_pos = transform.translation + attack_offset;

        commands.spawn((
            Transform::from_translation(attack_pos),
            Visibility::default(),
            Attack::new(10.0, 150.0 * facing_dir, 0.1),
            AttackType::Basic,
            Collider::new(Vec2::new(40.0, 32.0)),
            PlayerAttack,
            Name::new("PlayerAttack"),
        ));
    }
}

/// Update attack cooldown timers
pub fn update_attack_cooldown(time: Res<Time>, mut query: Query<&mut AttackCooldown>) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

/// Check AABB collision between two colliders
#[inline]
fn check_attack_collision(
    attack_pos: Vec2,
    attack_collider: &Collider,
    target_pos: Vec2,
    target_collider: &Collider,
) -> bool {
    let a_min = attack_pos + attack_collider.offset - attack_collider.size / 2.0;
    let a_max = attack_pos + attack_collider.offset + attack_collider.size / 2.0;
    let b_min = target_pos + target_collider.offset - target_collider.size / 2.0;
    let b_max = target_pos + target_collider.offset + target_collider.size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

/// Detect collisions between attacks and targets
#[allow(clippy::type_complexity)]
pub fn attack_collision_system(
    mut attack_query: Query<(
        Entity,
        &Transform,
        &Collider,
        &mut Attack,
        Option<&PlayerAttack>,
    )>,
    player_query: Query<(Entity, &Transform, &Collider), (With<Player>, With<PlayerHealth>)>,
    enemy_query: Query<(Entity, &Transform, &Collider), With<EnemyHealth>>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    for (attack_entity, attack_transform, attack_collider, mut attack, player_attack) in
        attack_query.iter_mut()
    {
        let attack_pos = attack_transform.translation.truncate();

        if player_attack.is_some() {
            // Player attack -> check against enemies
            for (enemy_entity, enemy_transform, enemy_collider) in enemy_query.iter() {
                // Skip if already hit
                if attack.hit_entities.contains(&enemy_entity) {
                    continue;
                }

                let enemy_pos = enemy_transform.translation.truncate();
                if check_attack_collision(attack_pos, attack_collider, enemy_pos, enemy_collider) {
                    // Mark as hit to prevent multi-hit
                    attack.hit_entities.insert(enemy_entity);

                    // Fire damage event
                    damage_events.write(DamageEvent {
                        target: enemy_entity,
                        damage: attack.damage,
                        knockback: Vec2::new(attack.knockback, 50.0),
                        attacker: Some(attack_entity),
                    });

                    info!(
                        "Player attack hit enemy! Damage: {}, Knockback: {}",
                        attack.damage, attack.knockback
                    );
                }
            }
        } else {
            // Enemy attack -> check against player
            for (player_entity, player_transform, player_collider) in player_query.iter() {
                // Skip if already hit
                if attack.hit_entities.contains(&player_entity) {
                    continue;
                }

                let player_pos = player_transform.translation.truncate();
                if check_attack_collision(attack_pos, attack_collider, player_pos, player_collider)
                {
                    // Mark as hit
                    attack.hit_entities.insert(player_entity);

                    // Fire damage event
                    damage_events.write(DamageEvent {
                        target: player_entity,
                        damage: attack.damage,
                        knockback: Vec2::new(attack.knockback, 50.0),
                        attacker: Some(attack_entity),
                    });
                }
            }
        }
    }
}

/// Despawn attacks when their lifetime expires
pub fn attack_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Attack)>,
) {
    for (entity, mut attack) in query.iter_mut() {
        attack.lifetime.tick(time.delta());

        if attack.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_creation() {
        let attack = Attack::new(10.0, 100.0, 0.5);
        assert_eq!(attack.damage, 10.0);
        assert_eq!(attack.knockback, 100.0);
        assert!(attack.hit_entities.is_empty());
    }

    #[test]
    fn test_attack_cooldown() {
        let cooldown = AttackCooldown::new(0.5);
        assert!(!cooldown.can_attack()); // Timer starts at 0, needs to tick to finish
    }

    #[test]
    fn test_attack_collision_detection() {
        let attack_collider = Collider::new(Vec2::new(32.0, 32.0));
        let target_collider = Collider::new(Vec2::new(32.0, 32.0));

        // Overlapping
        assert!(check_attack_collision(
            Vec2::new(0.0, 0.0),
            &attack_collider,
            Vec2::new(16.0, 0.0),
            &target_collider
        ));

        // Not overlapping
        assert!(!check_attack_collision(
            Vec2::new(0.0, 0.0),
            &attack_collider,
            Vec2::new(100.0, 0.0),
            &target_collider
        ));
    }
}
