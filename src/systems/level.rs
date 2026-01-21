//! Level systems
//!
//! Systems for camera follow, parallax backgrounds, goal/checkpoint detection,
//! and tile collision handling.

use bevy::prelude::*;

use crate::components::level::{Checkpoint, CollisionTile, Goal, LevelEntity, ParallaxLayer};
use crate::components::{Collider, Player};
use crate::events::{CheckpointActivatedEvent, GoalReachedEvent};

// ============================================================================
// Camera Systems
// ============================================================================

/// System to make the camera follow the player smoothly
pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    // Smooth follow with time-based lerp
    let target = player_transform.translation.truncate();
    let current = camera_transform.translation.truncate();

    // Lerp speed (higher = faster follow)
    let lerp_speed = 5.0;
    let t = (lerp_speed * time.delta_secs()).min(1.0);

    let new_pos = current.lerp(target, t);
    camera_transform.translation.x = new_pos.x;
    camera_transform.translation.y = new_pos.y;
}

// ============================================================================
// Parallax Systems
// ============================================================================

/// System to update parallax background positions based on camera movement
pub fn parallax_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut parallax_query: Query<(&mut Transform, &ParallaxLayer), Without<Camera2d>>,
) {
    let Ok(camera) = camera_query.single() else {
        return;
    };

    for (mut transform, layer) in &mut parallax_query {
        // Calculate parallax offset based on camera position and layer speed
        // Lower speed values create more distant/slower moving backgrounds
        let parallax_offset_x = camera.translation.x * (1.0 - layer.speed);
        let parallax_offset_y = camera.translation.y * (1.0 - layer.speed);

        transform.translation.x = layer.base_x + parallax_offset_x;
        transform.translation.y = layer.base_y + parallax_offset_y;
    }
}

/// Spawn parallax background layers
pub fn spawn_parallax_backgrounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Background layer (furthest back, slowest)
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/environments/sunny_land/layers/background.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -100.0),
        ParallaxLayer::with_position(0.2, 0, 0.0, 0.0),
        LevelEntity,
        Name::new("Parallax Background"),
    ));

    // Middleground layer (closer, faster)
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/environments/sunny_land/layers/middleground.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -50.0),
        ParallaxLayer::with_position(0.5, 1, 0.0, 0.0),
        LevelEntity,
        Name::new("Parallax Middleground"),
    ));
}

// ============================================================================
// Goal Detection Systems
// ============================================================================

/// System to detect when the player reaches the goal
pub fn goal_detection_system(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    goal_query: Query<&Transform, With<Goal>>,
    mut goal_events: MessageWriter<GoalReachedEvent>,
) {
    let Ok((player_transform, player_collider)) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let player_half_size = player_collider.size / 2.0;

    for goal_transform in &goal_query {
        let goal_pos = goal_transform.translation.truncate();
        // Default goal size (can be customized via LDtk entity fields)
        let goal_half_size = Vec2::new(16.0, 32.0);

        // AABB collision check
        if player_pos.x - player_half_size.x < goal_pos.x + goal_half_size.x
            && player_pos.x + player_half_size.x > goal_pos.x - goal_half_size.x
            && player_pos.y - player_half_size.y < goal_pos.y + goal_half_size.y
            && player_pos.y + player_half_size.y > goal_pos.y - goal_half_size.y
        {
            info!("Player reached the goal!");
            goal_events.write(GoalReachedEvent);
        }
    }
}

// ============================================================================
// Checkpoint Systems
// ============================================================================

/// System to detect and activate checkpoints when the player passes through
pub fn checkpoint_detection_system(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    mut checkpoint_query: Query<(&Transform, &mut Checkpoint)>,
    mut checkpoint_events: MessageWriter<CheckpointActivatedEvent>,
) {
    let Ok((player_transform, player_collider)) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let player_half_size = player_collider.size / 2.0;

    for (checkpoint_transform, mut checkpoint) in &mut checkpoint_query {
        // Skip already activated checkpoints
        if checkpoint.activated {
            continue;
        }

        let checkpoint_pos = checkpoint_transform.translation.truncate();
        // Default checkpoint size
        let checkpoint_half_size = Vec2::new(16.0, 32.0);

        // AABB collision check
        if player_pos.x - player_half_size.x < checkpoint_pos.x + checkpoint_half_size.x
            && player_pos.x + player_half_size.x > checkpoint_pos.x - checkpoint_half_size.x
            && player_pos.y - player_half_size.y < checkpoint_pos.y + checkpoint_half_size.y
            && player_pos.y + player_half_size.y > checkpoint_pos.y - checkpoint_half_size.y
        {
            checkpoint.activated = true;
            info!("Checkpoint {} activated!", checkpoint.id);
            checkpoint_events.write(CheckpointActivatedEvent {
                checkpoint_id: checkpoint.id,
                position: checkpoint_pos,
            });
        }
    }
}

/// System to update player spawn point when a checkpoint is activated
pub fn update_spawn_from_checkpoint(
    mut checkpoint_events: MessageReader<CheckpointActivatedEvent>,
    mut spawn_point: ResMut<crate::combat::PlayerSpawnPoint>,
) {
    for event in checkpoint_events.read() {
        // Convert Vec2 to Vec3 (keeping z at 0)
        spawn_point.position = event.position.extend(0.0);
        info!(
            "Player spawn point updated to checkpoint {} at {:?}",
            event.checkpoint_id, event.position
        );
    }
}

// ============================================================================
// Tile Collision Systems
// ============================================================================

/// System to add colliders to collision tiles spawned from LDtk IntGrid
pub fn setup_tile_colliders(
    mut commands: Commands,
    collision_tiles: Query<(Entity, &Transform), (With<CollisionTile>, Without<Collider>)>,
) {
    for (entity, _transform) in &collision_tiles {
        // Add collider component to collision tiles
        // Tile size is 8x8 for test_stage.ldtk
        commands
            .entity(entity)
            .insert(Collider::new(Vec2::splat(8.0)));
    }
}

// ============================================================================
// Level Cleanup Systems
// ============================================================================

/// Clean up all level entities when leaving the level
pub fn cleanup_level(mut commands: Commands, level_entities: Query<Entity, With<LevelEntity>>) {
    for entity in &level_entities {
        commands.entity(entity).despawn();
    }
    info!("Level entities cleaned up");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallax_layer_creation() {
        let layer = ParallaxLayer::new(0.5, 1);
        assert_eq!(layer.speed, 0.5);
        assert_eq!(layer.layer_index, 1);
        assert_eq!(layer.base_x, 0.0);
        assert_eq!(layer.base_y, 0.0);
    }

    #[test]
    fn test_parallax_layer_with_position() {
        let layer = ParallaxLayer::with_position(0.3, 2, 100.0, 50.0);
        assert_eq!(layer.speed, 0.3);
        assert_eq!(layer.layer_index, 2);
        assert_eq!(layer.base_x, 100.0);
        assert_eq!(layer.base_y, 50.0);
    }
}
