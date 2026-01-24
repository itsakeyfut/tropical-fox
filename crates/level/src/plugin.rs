//! Level plugin
//!
//! Handles LDtk level loading, tile rendering, and level-specific systems.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use tropical_fox_common::InGameState;

use crate::bundles::{
    CheckpointBundle, GoalBundle, OneWayPlatformBundle, SolidGroundBundle, SolidWallBundle,
};
use crate::components::ActiveCheckpoint;
use crate::config::{CurrentLevel, load_levels_config_optional};
use crate::systems::{
    check_checkpoint_collision, check_goal_collision, handle_stage_transition,
    initialize_parallax_positions, spawn_background_layers, update_parallax,
};

/// Plugin that manages level loading and rendering
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Add LDtk plugin
        app.add_plugins(LdtkPlugin);

        // Register IntGrid cell bundles
        app.register_ldtk_int_cell::<SolidGroundBundle>(1);
        app.register_ldtk_int_cell::<SolidWallBundle>(2);
        app.register_ldtk_int_cell::<OneWayPlatformBundle>(3);

        // Register LDtk entity bundles
        app.register_ldtk_entity::<GoalBundle>("Goal");
        app.register_ldtk_entity::<CheckpointBundle>("Checkpoint");

        // Configure LDtk settings
        app.insert_resource(LevelSelection::index(0));

        // Initialize resources
        app.insert_resource(CurrentLevel::default());
        app.insert_resource(ActiveCheckpoint::default());

        // Load level configuration in Startup
        app.add_systems(Startup, load_level_config);

        // Load level and background when entering StagePlay state
        app.add_systems(
            OnEnter(InGameState::StagePlay),
            (spawn_ldtk_world, spawn_background_layers),
        );

        // Update parallax, check checkpoint and goal collision in Update schedule
        app.add_systems(
            Update,
            (
                initialize_parallax_positions,
                update_parallax,
                check_checkpoint_collision,
                check_goal_collision,
            )
                .run_if(in_state(InGameState::StagePlay)),
        );

        // Handle stage transition when in StageTransition state
        app.add_systems(
            OnEnter(InGameState::StageTransition),
            handle_stage_transition,
        );

        // Cleanup level when exiting StagePlay state
        app.add_systems(OnExit(InGameState::StagePlay), cleanup_level);
    }
}

/// Load level configuration from RON file
fn load_level_config() {
    if let Some(config) = load_levels_config_optional("assets/config/levels.ron") {
        info!(
            "Loaded levels config: {} levels, default: {}",
            config.levels.len(),
            config.default_level
        );
    } else {
        warn!("Could not load levels.ron, using defaults");
    }
}

/// Spawn the LDtk world
fn spawn_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading LDtk level");

    // Load the LDtk project file
    let ldtk_handle: Handle<LdtkProject> = asset_server.load("levels/test.ldtk");

    // Spawn the LDtk world entity
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: ldtk_handle,
        },
        ..default()
    });

    info!("LDtk world spawned");
}

/// Cleanup level entities when exiting StagePlay
fn cleanup_level(
    _commands: Commands,
    _ldtk_worlds: Query<Entity, (With<Transform>, With<GlobalTransform>)>,
) {
    info!("Cleaning up level entities");

    // For now, skip cleanup - LDtk handles this internally
    // TODO: Implement proper cleanup in Phase 2

    info!("Level cleanup complete (placeholder)");
}
