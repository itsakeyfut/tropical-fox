//! Checkpoint systems
//!
//! Handles checkpoint activation and player respawn

use bevy::prelude::*;
use tropical_fox_common::{Collider, Player};

use crate::components::{ActiveCheckpoint, Checkpoint};

/// Check for player collision with checkpoints and activate them
pub fn check_checkpoint_collision(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    mut checkpoint_query: Query<(&Transform, &mut Checkpoint, &Collider)>,
    mut active_checkpoint: ResMut<ActiveCheckpoint>,
) {
    let Some((player_transform, player_collider)) = player_query.iter().next() else {
        return;
    };

    let player_pos = player_transform.translation;
    let player_half_size = player_collider.size / 2.0;

    for (checkpoint_transform, mut checkpoint, checkpoint_collider) in checkpoint_query.iter_mut() {
        // Skip already activated checkpoints
        if checkpoint.activated {
            continue;
        }

        let checkpoint_pos = checkpoint_transform.translation;
        let checkpoint_half_size = checkpoint_collider.size / 2.0;

        // AABB collision detection
        let collision = player_pos.x + player_half_size.x
            > checkpoint_pos.x - checkpoint_half_size.x
            && player_pos.x - player_half_size.x < checkpoint_pos.x + checkpoint_half_size.x
            && player_pos.y + player_half_size.y > checkpoint_pos.y - checkpoint_half_size.y
            && player_pos.y - player_half_size.y < checkpoint_pos.y + checkpoint_half_size.y;

        if collision {
            info!(
                "✓ Checkpoint activated at position ({:.0}, {:.0})",
                checkpoint.spawn_position.x, checkpoint.spawn_position.y
            );

            // Activate this checkpoint
            checkpoint.activated = true;

            // Update active checkpoint resource
            active_checkpoint.respawn_position = checkpoint.spawn_position;

            // Visual/audio feedback would go here
            // - Play sound effect
            // - Trigger particle effect
            // - Change sprite/animation
        }
    }
}

/// Respawn player at active checkpoint (triggered by death event)
/// This is a placeholder system - actual death handling would be in combat system
pub fn respawn_at_checkpoint(
    mut player_query: Query<&mut Transform, With<Player>>,
    active_checkpoint: Res<ActiveCheckpoint>,
    // In full implementation, this would listen to a DeathEvent
) {
    // This system would be triggered by a death event in the combat system
    // For now, it's just a placeholder showing how respawn would work

    if let Some(mut player_transform) = player_query.iter_mut().next() {
        player_transform.translation = active_checkpoint.respawn_position;
        info!(
            "Player respawned at checkpoint: ({:.0}, {:.0})",
            active_checkpoint.respawn_position.x, active_checkpoint.respawn_position.y
        );
    }
}
