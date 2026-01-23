//! Game systems
//!
//! This module will contain all ECS systems used throughout the game.

pub mod animation;
pub mod physics;
pub mod player;

#[cfg(debug_assertions)]
pub mod hot_reload;

pub use animation::{player_animation_controller, process_animation_events, update_animations};
pub use physics::{apply_gravity, update_position};
pub use player::{
    flip_sprite_by_facing, ground_collision, player_dash, player_horizontal_movement, player_jump,
    update_dash, variable_jump_height, wall_collision, wall_jump, wall_slide,
};

#[cfg(debug_assertions)]
pub use hot_reload::{
    apply_bosses_config_reload, apply_enemies_config_reload, apply_game_settings_reload,
    apply_players_config_reload,
};
