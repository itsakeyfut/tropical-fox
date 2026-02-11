//! Physics systems
//!
//! Basic physics simulation including gravity and velocity.

use bevy::prelude::*;
use tropical_fox_common::{Gravity, PhysicsConfig, Velocity};

/// Apply gravity to entities with Gravity and Velocity components
pub fn apply_gravity(
    time: Res<Time>,
    physics_config: Res<PhysicsConfig>,
    mut query: Query<(&Gravity, &mut Velocity)>,
) {
    let delta = time.delta_secs();
    let base_gravity = physics_config.gravity;
    let terminal_velocity = physics_config.terminal_velocity;

    for (gravity, mut velocity) in query.iter_mut() {
        velocity.y += base_gravity * gravity.scale * delta;
        // Clamp velocity to terminal velocity (prevent falling faster than terminal_velocity)
        velocity.y = velocity.y.max(terminal_velocity);
    }
}

/// Update entity positions based on their velocity
pub fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let delta = time.delta_secs();

    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * delta;
        transform.translation.y += velocity.y * delta;
    }
}
