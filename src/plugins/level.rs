//! Level plugin
//!
//! Handles level loading, tilemap rendering, collision setup,
//! parallax backgrounds, and goal/checkpoint systems.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::level::{
    CheckpointBundle, GoalBundle, GroundCollisionBundle, PlayerSpawnBundle,
};
use crate::events::{CheckpointActivatedEvent, GoalReachedEvent, LevelLoadedEvent};
use crate::game_state::GameState;
use crate::systems::level::{
    camera_follow_player, checkpoint_detection_system, cleanup_level, goal_detection_system,
    setup_tile_colliders, spawn_parallax_backgrounds, update_spawn_from_checkpoint,
};

/// Plugin that manages level loading and level-related systems
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Register LDtk plugin
        app.add_plugins(LdtkPlugin);

        // Register level events (messages in Bevy 0.17)
        app.add_message::<GoalReachedEvent>();
        app.add_message::<CheckpointActivatedEvent>();
        app.add_message::<LevelLoadedEvent>();

        // Configure LDtk settings
        app.insert_resource(LevelSelection::index(0));

        // Register LDtk IntGrid collision tiles
        // IntGrid value 1 = walls (from test_stage.ldtk)
        app.register_ldtk_int_cell::<GroundCollisionBundle>(1);

        // Register LDtk entities
        app.register_ldtk_entity::<GoalBundle>("Goal");
        app.register_ldtk_entity::<CheckpointBundle>("Checkpoint");
        app.register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn");

        // Level setup systems - run when entering InGame state
        // Order: spawn_level first, then backgrounds (backgrounds need to be behind tilemap)
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_level, spawn_parallax_backgrounds).chain(),
        );

        // Level cleanup - run when exiting InGame state
        app.add_systems(OnExit(GameState::InGame), cleanup_level);

        // Runtime systems
        app.add_systems(
            Update,
            (
                camera_follow_player,
                goal_detection_system,
                checkpoint_detection_system,
                update_spawn_from_checkpoint,
            )
                .into_configs()
                .run_if(in_state(GameState::InGame)),
        );

        // Tile collision setup (runs in FixedUpdate to catch new tiles)
        app.add_systems(
            FixedUpdate,
            setup_tile_colliders.run_if(in_state(GameState::InGame)),
        );
    }
}

/// Spawn the LDtk level world
fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<crate::config::GameSettings>,
) {
    info!("Loading level from LDtk file...");

    // Get stage settings from config
    let stage = &settings.stage;

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: LdtkProjectHandle {
                handle: asset_server
                    .load("graphics/environments/test_stage/levels/test_stage.ldtk"),
            },
            transform: Transform {
                translation: Vec3::new(stage.position_x, stage.position_y, 0.0),
                scale: Vec3::splat(stage.scale),
                ..default()
            },
            ..default()
        },
        crate::components::level::LevelEntity,
        Name::new("LDtk World"),
    ));
}
