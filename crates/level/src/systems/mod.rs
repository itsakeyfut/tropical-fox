pub mod checkpoint;
pub mod goal;
pub mod parallax;

pub use checkpoint::check_checkpoint_collision;
pub use goal::{check_goal_collision, handle_stage_transition};
pub use parallax::{initialize_parallax_positions, spawn_background_layers, update_parallax};
