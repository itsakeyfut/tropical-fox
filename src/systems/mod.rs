//! Game systems
//!
//! This module will contain all ECS systems used throughout the game.

pub mod physics;
pub mod player;

pub use physics::{apply_gravity, update_position};
pub use player::{
    flip_sprite_by_facing, ground_collision, player_horizontal_movement, player_jump,
};
