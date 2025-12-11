//! Player-related systems
//!
//! Handles player input, movement, jumping, and sprite updates.

use bevy::prelude::*;

use crate::components::{Collider, Ground, GroundDetection, Player, PlayerStats, Velocity};

/// Handle horizontal player movement based on keyboard input
pub fn player_horizontal_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &PlayerStats)>,
    time: Res<Time>,
) {
    for (mut velocity, mut player, stats) in &mut query {
        let mut input = 0.0;

        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            input -= 1.0;
            player.facing_right = false;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            input += 1.0;
            player.facing_right = true;
        }

        let target_velocity = input * stats.move_speed;

        // Smooth acceleration/deceleration using linear interpolation
        if input != 0.0 {
            velocity.x = velocity
                .x
                .lerp(target_velocity, stats.acceleration * time.delta_secs());
        } else {
            velocity.x = velocity.x.lerp(0.0, stats.deceleration * time.delta_secs());
        }
    }
}

/// Handle player jump input
pub fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection, &PlayerStats), With<Player>>,
) {
    for (mut velocity, ground, stats) in &mut query {
        if ground.is_grounded && keyboard.just_pressed(KeyCode::Space) {
            velocity.y = stats.jump_force;
        }
    }
}

/// Check AABB collision between two colliders
#[inline]
fn check_aabb_collision(
    pos_a: Vec2,
    collider_a: &Collider,
    pos_b: Vec2,
    collider_b: &Collider,
) -> bool {
    let a_min = pos_a + collider_a.offset - collider_a.size / 2.0;
    let a_max = pos_a + collider_a.offset + collider_a.size / 2.0;
    let b_min = pos_b + collider_b.offset - collider_b.size / 2.0;
    let b_max = pos_b + collider_b.offset + collider_b.size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

/// Handle collision between player and ground
#[allow(clippy::type_complexity)]
pub fn ground_collision(
    mut player_query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut GroundDetection,
            &Collider,
        ),
        With<Player>,
    >,
    ground_query: Query<(&Transform, &Collider), (With<Ground>, Without<Player>)>,
) {
    for (mut player_transform, mut velocity, mut ground_detection, player_collider) in
        &mut player_query
    {
        ground_detection.is_grounded = false;

        for (ground_transform, ground_collider) in &ground_query {
            let player_pos = player_transform.translation.truncate();
            let ground_pos = ground_transform.translation.truncate();

            // Check AABB collision
            if check_aabb_collision(player_pos, player_collider, ground_pos, ground_collider) {
                // Collision from above (player is falling onto ground)
                if velocity.y < 0.0 {
                    // Position player on top of the ground
                    player_transform.translation.y = ground_transform.translation.y
                        + ground_collider.size.y / 2.0
                        + player_collider.size.y / 2.0;
                    velocity.y = 0.0;
                    ground_detection.is_grounded = true;
                }
            }
        }
    }
}

/// Flip sprite based on player facing direction
pub fn flip_sprite_by_facing(mut query: Query<(&Player, &mut Sprite)>) {
    for (player, mut sprite) in &mut query {
        sprite.flip_x = !player.facing_right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_collision_overlapping() {
        let collider_a = Collider::new(Vec2::new(32.0, 32.0));
        let collider_b = Collider::new(Vec2::new(32.0, 32.0));

        // Test overlapping colliders
        assert!(check_aabb_collision(
            Vec2::new(0.0, 0.0),
            &collider_a,
            Vec2::new(16.0, 0.0),
            &collider_b
        ));
    }

    #[test]
    fn test_aabb_collision_separated() {
        let collider_a = Collider::new(Vec2::new(32.0, 32.0));
        let collider_b = Collider::new(Vec2::new(32.0, 32.0));

        // Test separated colliders
        assert!(!check_aabb_collision(
            Vec2::new(0.0, 0.0),
            &collider_a,
            Vec2::new(100.0, 0.0),
            &collider_b
        ));
    }

    #[test]
    fn test_aabb_collision_touching() {
        let collider_a = Collider::new(Vec2::new(32.0, 32.0));
        let collider_b = Collider::new(Vec2::new(32.0, 32.0));

        // Test touching colliders (should NOT collide when exactly touching)
        // This prevents physics jitter in edge cases
        assert!(!check_aabb_collision(
            Vec2::new(0.0, 0.0),
            &collider_a,
            Vec2::new(32.0, 0.0),
            &collider_b
        ));
    }

    #[test]
    fn test_aabb_collision_barely_overlapping() {
        let collider_a = Collider::new(Vec2::new(32.0, 32.0));
        let collider_b = Collider::new(Vec2::new(32.0, 32.0));

        // Test barely overlapping colliders (should collide)
        assert!(check_aabb_collision(
            Vec2::new(0.0, 0.0),
            &collider_a,
            Vec2::new(31.0, 0.0),
            &collider_b
        ));
    }
}
