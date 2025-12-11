//! Player-related systems
//!
//! Handles player input, movement, jumping, and sprite updates.

use bevy::prelude::*;

use crate::components::{Collider, Ground, GroundDetection, Player, PlayerStats, Velocity, Wall};

/// Handle horizontal player movement based on keyboard input
pub fn player_horizontal_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &PlayerStats, &GroundDetection)>,
    time: Res<Time>,
) {
    for (mut velocity, mut player, stats, ground) in &mut query {
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

        // Choose deceleration based on whether player is grounded
        let deceleration = if ground.is_grounded {
            stats.deceleration
        } else {
            stats.air_deceleration
        };

        // Smooth acceleration/deceleration using linear interpolation
        if input != 0.0 {
            velocity.x = velocity
                .x
                .lerp(target_velocity, stats.acceleration * time.delta_secs());
        } else {
            velocity.x = velocity.x.lerp(0.0, deceleration * time.delta_secs());
        }
    }
}

/// Handle player jump input with coyote time and jump buffering
pub fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &GroundDetection, &PlayerStats)>,
    time: Res<Time>,
) {
    for (mut velocity, mut player, ground, stats) in &mut query {
        // Update jump buffer timer if jump was pressed
        if keyboard.just_pressed(KeyCode::Space) {
            player.jump_buffer_timer = stats.jump_buffer_time;
        }

        // Countdown jump buffer timer
        if player.jump_buffer_timer > 0.0 {
            player.jump_buffer_timer -= time.delta_secs();
        }

        // Can jump if: grounded OR within coyote time
        let can_jump = ground.is_grounded || player.coyote_timer > 0.0;

        // Perform jump if we can jump and have a buffered jump input
        if can_jump && player.jump_buffer_timer > 0.0 {
            velocity.y = stats.jump_force;
            player.jump_buffer_timer = 0.0; // Consume the buffered jump
            player.coyote_timer = 0.0; // Consume coyote time
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
            &mut Player,
            &PlayerStats,
            &Collider,
        ),
        With<Player>,
    >,
    ground_query: Query<(&Transform, &Collider), (With<Ground>, Without<Player>)>,
    time: Res<Time>,
) {
    for (
        mut player_transform,
        mut velocity,
        mut ground_detection,
        mut player,
        stats,
        player_collider,
    ) in &mut player_query
    {
        let was_grounded = ground_detection.is_grounded;
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

        // Manage coyote time and dash reset
        if ground_detection.is_grounded {
            // Reset coyote timer when on ground
            player.coyote_timer = stats.coyote_time;
            // Reset dash count when on ground
            player.dashes_remaining = stats.max_air_dashes;
        } else if was_grounded {
            // Just left ground - only start coyote time if it wasn't consumed by a jump
            // (If player jumped, coyote_timer is already 0)
            if player.coyote_timer > 0.0 {
                player.coyote_timer = stats.coyote_time;
            }
        } else {
            // In air - countdown coyote timer
            player.coyote_timer = (player.coyote_timer - time.delta_secs()).max(0.0);
        }
    }
}

/// Handle variable jump height (cut jump short when button is released)
pub fn variable_jump_height(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &PlayerStats), With<Player>>,
) {
    for (mut velocity, stats) in &mut query {
        // If player just released jump button while still moving upward, cut the jump short
        if keyboard.just_released(KeyCode::Space) && velocity.y > 0.0 {
            velocity.y *= stats.jump_cut_multiplier;
        }
    }
}

/// Handle player dash input
pub fn player_dash(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &PlayerStats)>,
) {
    for (mut player, stats) in &mut query {
        // Check if dash button is pressed (Shift key)
        if keyboard.just_pressed(KeyCode::ShiftLeft) || keyboard.just_pressed(KeyCode::ShiftRight) {
            // Can only dash if we have dashes remaining
            if player.dashes_remaining > 0 {
                // Get dash direction from arrow keys or WASD
                let mut dash_dir = Vec2::ZERO;

                if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
                    dash_dir.x -= 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
                    dash_dir.x += 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
                    dash_dir.y += 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
                    dash_dir.y -= 1.0;
                }

                // If no direction input, dash in facing direction
                if dash_dir.length_squared() == 0.0 {
                    dash_dir.x = if player.facing_right { 1.0 } else { -1.0 };
                }

                // Normalize direction and start dash
                dash_dir = dash_dir.normalize_or_zero();
                if dash_dir.length_squared() > 0.0 {
                    player.dash_direction = dash_dir;
                    player.dash_timer = stats.dash_duration;
                    player.dashes_remaining -= 1;
                }
            }
        }
    }
}

/// Update dash state and apply dash velocity
pub fn update_dash(mut query: Query<(&mut Velocity, &mut Player, &PlayerStats)>, time: Res<Time>) {
    for (mut velocity, mut player, stats) in &mut query {
        if player.dash_timer > 0.0 {
            // Apply dash velocity
            velocity.x = player.dash_direction.x * stats.dash_speed;
            velocity.y = player.dash_direction.y * stats.dash_speed;

            // Countdown dash timer
            player.dash_timer -= time.delta_secs();

            if player.dash_timer <= 0.0 {
                // Dash ended
                player.dash_timer = 0.0;
            }
        }
    }
}

/// Handle collision detection between player and walls
#[allow(clippy::type_complexity)]
pub fn wall_collision(
    mut player_query: Query<
        (
            &mut Transform,
            &Collider,
            &mut Player,
            &mut Velocity,
            &GroundDetection,
        ),
        With<Player>,
    >,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Player>)>,
) {
    for (mut player_transform, player_collider, mut player, mut velocity, ground) in
        &mut player_query
    {
        player.wall_contact = 0;

        // Only detect wall contact if player is in air (not grounded)
        if !ground.is_grounded {
            for (wall_transform, wall_collider) in &wall_query {
                let player_pos = player_transform.translation.truncate();
                let wall_pos = wall_transform.translation.truncate();

                // Check AABB collision
                if check_aabb_collision(player_pos, player_collider, wall_pos, wall_collider) {
                    // Determine which side of the wall the player is on
                    let player_center_x = player_pos.x + player_collider.offset.x;
                    let wall_center_x = wall_pos.x + wall_collider.offset.x;

                    if player_center_x < wall_center_x {
                        // Player is on the left side of the wall (touching right side)
                        player.wall_contact = 1; // Wall on right

                        // Push player to the left of the wall
                        let wall_left_edge =
                            wall_pos.x + wall_collider.offset.x - wall_collider.size.x / 2.0;
                        let player_right_edge =
                            player_collider.offset.x + player_collider.size.x / 2.0;
                        player_transform.translation.x = wall_left_edge - player_right_edge;

                        // Stop horizontal movement into the wall
                        if velocity.x > 0.0 {
                            velocity.x = 0.0;
                        }
                    } else {
                        // Player is on the right side of the wall (touching left side)
                        player.wall_contact = -1; // Wall on left

                        // Push player to the right of the wall
                        let wall_right_edge =
                            wall_pos.x + wall_collider.offset.x + wall_collider.size.x / 2.0;
                        let player_left_edge =
                            player_collider.offset.x - player_collider.size.x / 2.0;
                        player_transform.translation.x = wall_right_edge - player_left_edge;

                        // Stop horizontal movement into the wall
                        if velocity.x < 0.0 {
                            velocity.x = 0.0;
                        }
                    }

                    break; // Only need to detect one wall at a time
                }
            }
        }
    }
}

/// Apply wall sliding physics
pub fn wall_slide(mut query: Query<(&mut Velocity, &Player, &PlayerStats, &GroundDetection)>) {
    for (mut velocity, player, stats, ground) in &mut query {
        // Only slide if touching a wall and not grounded
        if player.wall_contact != 0 && !ground.is_grounded && velocity.y < 0.0 {
            // Cap downward velocity to wall slide speed
            if velocity.y < stats.wall_slide_speed {
                velocity.y = stats.wall_slide_speed;
            }
        }
    }
}

/// Handle wall jump input
pub fn wall_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &PlayerStats, &GroundDetection)>,
) {
    for (mut velocity, mut player, stats, ground) in &mut query {
        // Can only wall jump if touching a wall and not grounded
        if player.wall_contact != 0 && !ground.is_grounded && keyboard.just_pressed(KeyCode::Space)
        {
            // Jump away from the wall
            let jump_dir_x = -player.wall_contact as f32; // Opposite direction of wall
            velocity.x = jump_dir_x * stats.wall_jump_force_x;
            velocity.y = stats.wall_jump_force_y;

            // Clear wall contact to prevent immediate re-attachment
            player.wall_contact = 0;

            // Reset dash count on wall jump (like Celeste)
            player.dashes_remaining = stats.max_air_dashes;
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
