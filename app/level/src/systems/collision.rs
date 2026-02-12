use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use tropical_fox_common::{Collider, Ground, Platform, Wall};

/// Spawns colliders for LDtk IntGrid tiles
pub fn spawn_tile_colliders(
    mut commands: Commands,
    int_grid_query: Query<(&IntGridCell, &Transform, Entity), Added<IntGridCell>>,
) {
    // LDtk tile size (8x8 pixels based on test.ldtk configuration)
    let tile_size = Vec2::splat(8.0);

    for (cell, _transform, entity) in int_grid_query.iter() {
        match cell.value {
            // 0 = Air (no collision)
            0 => {}
            // 1 = Ground (solid, walkable)
            1 => {
                commands.entity(entity).insert((
                    Ground,
                    Collider::new(tile_size),
                    Name::new("Ground Tile"),
                ));
            }
            // 2 = Wall (solid, blocking)
            2 => {
                commands.entity(entity).insert((
                    Wall,
                    Collider::new(tile_size),
                    Name::new("Wall Tile"),
                ));
            }
            // 3 = Platform (one-way, jump-through)
            3 => {
                commands.entity(entity).insert((
                    Platform,
                    Collider::new(tile_size),
                    Name::new("Platform Tile"),
                ));
            }
            _ => {
                warn!("Unknown IntGrid value: {}", cell.value);
            }
        }
    }
}
