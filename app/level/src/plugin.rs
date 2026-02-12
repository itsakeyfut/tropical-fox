use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::{LdtkSettings, LevelSpawnBehavior, SetClearColor};
use tropical_fox_common::GameState;

use crate::config::LevelMetadataConfig;
use crate::events::{CheckpointActivatedEvent, LevelCompleteEvent};
use crate::ldtk_entities::{CheckpointBundle, GoalBundle};
use crate::resources::{ActiveCheckpoint, CurrentLevel};
use crate::systems::{
    activate_checkpoint, detect_goal_reached, handle_level_complete, respawn_at_checkpoint,
    spawn_backgrounds, spawn_tile_colliders, update_checkpoint_visuals, update_parallax,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            // Configure LDtk settings
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: false,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            // Initialize asset types
            .init_asset::<LevelMetadataConfig>()
            // Register LDtk entity bundles
            .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
            .register_ldtk_entity::<GoalBundle>("Goal")
            // Add resources
            .init_resource::<ActiveCheckpoint>()
            .insert_resource(CurrentLevel {
                name: "stage_1".to_string(), // Default to stage_1
            })
            // Add events
            .add_message::<LevelCompleteEvent>()
            .add_message::<CheckpointActivatedEvent>()
            // Add systems
            .add_systems(OnEnter(GameState::InGame), spawn_level)
            .add_systems(
                Update,
                (
                    debug_ldtk_entities,
                    spawn_tile_colliders,
                    spawn_backgrounds,
                    update_parallax,
                    detect_goal_reached,
                    handle_level_complete,
                    activate_checkpoint,
                    update_checkpoint_visuals,
                    respawn_at_checkpoint,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning LDtk level");
    let ldtk_handle = asset_server.load("graphics/environments/test_stage/levels/test.ldtk");

    // Set which level to display (using index - 0 is the first level in the LDtk project)
    commands.insert_resource(LevelSelection::index(0));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: ldtk_handle,
        },
        ..Default::default()
    });
}

// Debug system to log when LDtk layers are spawned
fn debug_ldtk_entities(
    intgrid_query: Query<(&IntGridCell, &Transform), Added<IntGridCell>>,
    layer_query: Query<&Name, Added<LayerMetadata>>,
) {
    for name in layer_query.iter() {
        info!("LDtk layer spawned: {}", name);
    }

    for (cell, transform) in intgrid_query.iter() {
        info!(
            "IntGridCell spawned: value={}, pos=({}, {})",
            cell.value, transform.translation.x, transform.translation.y
        );
    }
}
