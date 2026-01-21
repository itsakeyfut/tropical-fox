//! Game systems
//!
//! This module will contain all ECS systems used throughout the game.

pub mod animation;
pub mod level;
pub mod physics;
pub mod player;

pub use animation::{player_animation_controller, process_animation_events, update_animations};
pub use level::{
    camera_follow_player, checkpoint_detection_system, cleanup_level, goal_detection_system,
    parallax_system, setup_tile_colliders, spawn_parallax_backgrounds,
    update_spawn_from_checkpoint,
};
pub use physics::{apply_gravity, update_position};
pub use player::{
    flip_sprite_by_facing, ground_collision, player_dash, player_horizontal_movement, player_jump,
    update_dash, variable_jump_height, wall_collision, wall_jump, wall_slide,
};
