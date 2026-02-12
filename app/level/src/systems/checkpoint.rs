use bevy::prelude::*;
use tropical_fox_common::{Collider, Player, PlayerDeathEvent};

use crate::components::Checkpoint;
use crate::events::CheckpointActivatedEvent;
use crate::resources::ActiveCheckpoint;

/// Checks AABB overlap between two colliders
#[inline]
fn check_aabb_overlap(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> bool {
    let pos_a = transform_a.translation.truncate();
    let pos_b = transform_b.translation.truncate();

    let a_min = pos_a + collider_a.offset - collider_a.size / 2.0;
    let a_max = pos_a + collider_a.offset + collider_a.size / 2.0;
    let b_min = pos_b + collider_b.offset - collider_b.size / 2.0;
    let b_max = pos_b + collider_b.offset + collider_b.size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

/// Activates checkpoints when player touches them
pub fn activate_checkpoint(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    mut checkpoint_query: Query<(&Transform, &Collider, &mut Checkpoint)>,
    mut active_checkpoint: ResMut<ActiveCheckpoint>,
    mut events: MessageWriter<CheckpointActivatedEvent>,
) {
    let Ok((player_transform, player_collider)) = player_query.single() else {
        return;
    };

    for (cp_transform, cp_collider, mut checkpoint) in checkpoint_query.iter_mut() {
        if !checkpoint.activated {
            // Create a larger trigger area for checkpoints
            let trigger_collider = Collider::new(cp_collider.size * 1.5);

            if check_aabb_overlap(
                player_transform,
                player_collider,
                cp_transform,
                &trigger_collider,
            ) {
                checkpoint.activated = true;
                active_checkpoint.checkpoint_id = Some(checkpoint.checkpoint_id.clone());
                active_checkpoint.position = cp_transform.translation;
                info!("Checkpoint '{}' activated", checkpoint.checkpoint_id);
                events.write(CheckpointActivatedEvent {
                    checkpoint_id: checkpoint.checkpoint_id.clone(),
                });
            }
        }
    }
}

/// Updates checkpoint visual appearance when activated
pub fn update_checkpoint_visuals(
    mut checkpoint_query: Query<(&Checkpoint, &mut Sprite), Changed<Checkpoint>>,
) {
    for (checkpoint, mut sprite) in checkpoint_query.iter_mut() {
        sprite.color = if checkpoint.activated {
            Color::srgb(0.0, 1.0, 0.0) // Green when active
        } else {
            Color::srgb(0.5, 0.5, 0.5) // Gray when inactive
        };
    }
}

/// Respawns player at active checkpoint when they die
pub fn respawn_at_checkpoint(
    mut player_query: Query<&mut Transform, With<Player>>,
    active_checkpoint: Res<ActiveCheckpoint>,
    mut events: MessageReader<PlayerDeathEvent>,
) {
    for _event in events.read() {
        if let Ok(mut player_transform) = player_query.single_mut() {
            player_transform.translation = active_checkpoint.position;
            info!("Respawning at {:?}", active_checkpoint.position);
        }
    }
}
