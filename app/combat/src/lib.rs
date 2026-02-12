//! Combat system
//!
//! This module implements the combat mechanics including:
//! - Health and damage
//! - Attack hitboxes
//! - Knockback
//! - Invincibility frames
//! - Death and respawn

use bevy::prelude::*;

pub mod attack;
pub mod damage;
pub mod effects;
pub mod health;

pub use attack::*;
pub use damage::*;
pub use effects::*;
pub use health::*;

use tropical_fox_common::{GameState, PlayerDeathEvent};

/// Event fired when an entity takes damage
#[derive(bevy::ecs::prelude::Message)]
pub struct DamageEvent {
    /// The entity receiving damage
    pub target: Entity,
    /// Amount of damage dealt
    pub damage: f32,
    /// Knockback force to apply
    pub knockback: Vec2,
    /// The entity that dealt the damage (if any)
    pub attacker: Option<Entity>,
}

/// Event fired when an entity dies
#[derive(bevy::ecs::prelude::Message)]
pub struct DeathEvent {
    /// The entity that died
    pub entity: Entity,
}

/// Event to trigger hit stop effect (brief pause on hit)
#[derive(bevy::ecs::prelude::Message)]
pub struct HitStopEvent {
    /// Duration of the hit stop in seconds
    pub duration_secs: f32,
}

/// Event to trigger screen shake
#[derive(bevy::ecs::prelude::Message)]
pub struct ScreenShakeEvent {
    /// Intensity of the shake (max pixel offset)
    pub intensity: f32,
    /// Duration of the shake in seconds
    pub duration_secs: f32,
}

/// Combat plugin that registers all combat-related systems
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(HitStopTimer::default());
        app.insert_resource(ScreenShake::default());

        // Events (Messages in Bevy 0.17)
        app.add_message::<DamageEvent>();
        app.add_message::<DeathEvent>();
        app.add_message::<HitStopEvent>();
        app.add_message::<ScreenShakeEvent>();
        app.add_message::<PlayerDeathEvent>();

        // Combat systems (execution order is important)
        app.add_systems(
            Update,
            (
                // Phase 1: Attack input and cooldown
                update_attack_cooldown,
                player_attack_input,
                // Phase 2: Hit detection
                attack_collision_system,
                attack_lifetime_system,
                // Phase 3: Damage processing
                damage_system,
                // Phase 4: Effects and status updates
                hitstop_system,
                screen_shake_system,
                invincibility_system,
                damage_flash_system,
                knockback_system,
                // Phase 5: Death processing
                death_system,
                player_death_system,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}
