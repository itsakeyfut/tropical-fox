//! Damage processing system
//!
//! Handles damage application, invincibility frames, knockback, and death.

use bevy::prelude::*;

use super::{
    DamageEvent, DeathEvent, Health, HitStopEvent, Lives, PlayerSpawnPoint, ScreenShakeEvent,
};
use tropical_fox_common::{GameState, Player, Velocity};

/// Invincibility component - entity is immune to damage while this is active
#[derive(Component, Debug)]
pub struct Invincibility {
    /// Timer counting down invincibility duration
    pub timer: Timer,
}

impl Invincibility {
    /// Create a new invincibility component with specified duration
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }
}

/// Damage flash effect - changes sprite color temporarily
#[derive(Component, Debug)]
pub struct DamageFlash {
    /// Timer for the flash effect
    pub timer: Timer,
    /// Color to flash to
    pub flash_color: Color,
    /// Original color to restore
    pub original_color: Color,
}

impl DamageFlash {
    /// Create a new damage flash effect
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
            flash_color: Color::srgb(1.0, 0.3, 0.3), // Red flash
            original_color: Color::WHITE,
        }
    }
}

/// Knockback component - applies force and disables control temporarily
#[derive(Component, Debug)]
pub struct Knockback {
    /// Force to apply (velocity override)
    pub force: Vec2,
    /// Duration of the knockback
    pub duration: Timer,
}

impl Knockback {
    /// Create a new knockback with specified force and duration
    pub fn new(force: Vec2, duration_secs: f32) -> Self {
        Self {
            force,
            duration: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }
}

/// Marker component for entities currently in knockback state (disables player input)
#[derive(Component, Debug, Default)]
pub struct InKnockback;

/// Process damage events
pub fn damage_system(
    mut commands: Commands,
    mut damage_events: MessageReader<DamageEvent>,
    mut health_query: Query<(&mut Health, Option<&Invincibility>, Option<&Sprite>)>,
    player_query: Query<Entity, With<Player>>,
    mut death_events: MessageWriter<DeathEvent>,
    mut hitstop_events: MessageWriter<HitStopEvent>,
    mut shake_events: MessageWriter<ScreenShakeEvent>,
) {
    for event in damage_events.read() {
        let Ok((mut health, invincibility, sprite)) = health_query.get_mut(event.target) else {
            continue;
        };

        // Skip damage if invincible
        if invincibility.is_some() {
            continue;
        }

        // Apply damage
        let is_dead = health.take_damage(event.damage);

        // Check if target is player for stronger effects
        let is_player = player_query.contains(event.target);

        // Trigger hit stop effect
        hitstop_events.write(HitStopEvent {
            duration_secs: if is_player { 0.08 } else { 0.05 },
        });

        // Trigger screen shake (stronger for player)
        shake_events.write(ScreenShakeEvent {
            intensity: if is_player { 8.0 } else { 4.0 },
            duration_secs: if is_player { 0.2 } else { 0.1 },
        });

        // Apply invincibility frames (player only)
        if is_player {
            commands
                .entity(event.target)
                .insert(Invincibility::new(1.0)); // 1 second of invincibility
        }

        // Apply damage flash effect
        let mut damage_flash = DamageFlash::new(0.15);
        if let Some(sprite) = sprite {
            damage_flash.original_color = sprite.color;
        }
        commands.entity(event.target).insert(damage_flash);

        // Apply knockback
        commands
            .entity(event.target)
            .insert((Knockback::new(event.knockback, 0.2), InKnockback));

        info!(
            "Entity {:?} took {} damage, health: {}/{}",
            event.target, event.damage, health.current, health.max
        );

        // Fire death event if health depleted
        if is_dead {
            death_events.write(DeathEvent {
                entity: event.target,
            });
        }
    }
}

/// Update invincibility timers and apply blinking effect
pub fn invincibility_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Invincibility,
        &mut Sprite,
        Option<&DamageFlash>,
    )>,
) {
    for (entity, mut invincibility, mut sprite, damage_flash) in query.iter_mut() {
        invincibility.timer.tick(time.delta());

        if invincibility.timer.is_finished() {
            // Remove invincibility
            commands.entity(entity).remove::<Invincibility>();
            sprite.color.set_alpha(1.0);
        } else {
            // Skip blinking if damage flash is active (avoid visual conflicts)
            if damage_flash.is_none() {
                // Blinking effect at 20Hz
                let blink = (invincibility.timer.elapsed_secs() * 20.0).sin() > 0.0;
                sprite.color.set_alpha(if blink { 0.5 } else { 1.0 });
            }
        }
    }
}

/// Update damage flash effect
pub fn damage_flash_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageFlash, &mut Sprite)>,
) {
    for (entity, mut flash, mut sprite) in query.iter_mut() {
        flash.timer.tick(time.delta());

        if flash.timer.is_finished() {
            // Remove flash and restore original color
            commands.entity(entity).remove::<DamageFlash>();
            sprite.color = flash.original_color;
            sprite.color.set_alpha(1.0);
        } else {
            // Interpolate between flash color and original based on progress
            let t = flash.timer.fraction();
            sprite.color = flash.original_color.mix(&flash.flash_color, 1.0 - t);

            // Also apply blinking during flash
            let blink = (flash.timer.elapsed_secs() * 20.0).sin() > 0.0;
            sprite.color.set_alpha(if blink { 0.5 } else { 1.0 });
        }
    }
}

/// Process knockback force application
pub fn knockback_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Knockback, &mut Velocity)>,
) {
    for (entity, mut knockback, mut velocity) in query.iter_mut() {
        knockback.duration.tick(time.delta());

        if knockback.duration.just_finished() {
            // Remove knockback components
            commands
                .entity(entity)
                .remove::<Knockback>()
                .remove::<InKnockback>();
        } else {
            // Apply knockback force (override velocity)
            velocity.x = knockback.force.x;
            velocity.y = knockback.force.y;

            // Decay knockback force over time (frame-rate independent)
            let decay_rate = 5.0;
            knockback.force *= (1.0 - decay_rate * time.delta_secs()).max(0.0);
        }
    }
}

/// Handle generic entity death
pub fn death_system(
    mut commands: Commands,
    mut death_events: MessageReader<DeathEvent>,
    player_query: Query<Entity, With<Player>>,
    name_query: Query<&Name>,
) {
    for event in death_events.read() {
        // Skip player (handled separately for respawn)
        if player_query.contains(event.entity) {
            continue;
        }

        if let Ok(name) = name_query.get(event.entity) {
            info!("{} died", name);
        } else {
            info!("Entity {:?} died", event.entity);
        }

        // Despawn the entity
        commands.entity(event.entity).despawn();
    }
}

/// Handle player death with respawn or game over
pub fn player_death_system(
    mut commands: Commands,
    mut death_events: MessageReader<DeathEvent>,
    mut player_query: Query<(&mut Health, &mut Transform, Option<&mut Lives>), With<Player>>,
    spawn_point: Option<Res<PlayerSpawnPoint>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in death_events.read() {
        let Ok((mut health, mut transform, lives)) = player_query.get_mut(event.entity) else {
            continue;
        };

        info!("Player died!");

        // Check if player has lives remaining
        if let Some(mut lives) = lives {
            if lives.lose_life() {
                // No lives remaining - game over
                info!("Game Over - No lives remaining");
                next_state.set(GameState::GameOver);
                return;
            }

            // Respawn player
            info!("Lives remaining: {}", lives.current);
        }

        // Respawn at spawn point
        let respawn_pos = spawn_point
            .as_ref()
            .map(|sp| sp.position)
            .unwrap_or(Vec3::new(0.0, 100.0, 0.0));

        transform.translation = respawn_pos;
        let max_health = health.max;
        health.heal(max_health); // Full heal on respawn

        // Remove any active damage effects
        commands
            .entity(event.entity)
            .remove::<Knockback>()
            .remove::<InKnockback>()
            .remove::<DamageFlash>()
            .insert(Invincibility::new(2.0)); // 2 seconds of invincibility after respawn

        info!("Player respawned at {:?}", respawn_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invincibility_creation() {
        let inv = Invincibility::new(1.0);
        assert!(!inv.timer.finished());
    }

    #[test]
    fn test_knockback_creation() {
        let kb = Knockback::new(Vec2::new(100.0, 50.0), 0.2);
        assert_eq!(kb.force.x, 100.0);
        assert_eq!(kb.force.y, 50.0);
        assert!(!kb.duration.finished());
    }

    #[test]
    fn test_damage_flash_creation() {
        let flash = DamageFlash::new(0.15);
        assert!(!flash.timer.finished());
    }
}
