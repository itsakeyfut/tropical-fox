use bevy::prelude::*;

/// Component for checkpoint entities
#[derive(Component, Debug)]
pub struct Checkpoint {
    pub checkpoint_id: String,
    pub activated: bool,
}

/// Component for level goal/exit entities
#[derive(Component, Debug, Default)]
pub struct LevelGoal {
    pub next_level: Option<String>,
}

/// Component for parallax scrolling background layers
#[derive(Component, Debug)]
pub struct ParallaxLayer {
    pub scroll_speed: f32,
    pub scroll_axis: ScrollAxis,
}

/// Axis along which parallax scrolling occurs
#[derive(Debug, Clone, Copy)]
pub enum ScrollAxis {
    Horizontal,
    Vertical,
    Both,
}
