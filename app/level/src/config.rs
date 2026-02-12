use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::components::ScrollAxis;

/// Level metadata configuration loaded from RON
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct LevelMetadataConfig {
    pub levels: HashMap<String, LevelMetadata>,
}

/// Metadata for a single level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelMetadata {
    pub ldtk_path: String,
    pub camera_scale: f32,
    pub level_offset: Vec2,
    pub backgrounds: Vec<BackgroundLayerDef>,
}

/// Background layer definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundLayerDef {
    pub texture_path: String,
    pub parallax_speed: f32,
    pub z_index: f32,
    pub scroll_axis: ScrollAxisDef,
}

/// Serializable version of ScrollAxis for RON
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScrollAxisDef {
    Horizontal,
    Vertical,
    Both,
}

impl From<ScrollAxisDef> for ScrollAxis {
    fn from(def: ScrollAxisDef) -> Self {
        match def {
            ScrollAxisDef::Horizontal => ScrollAxis::Horizontal,
            ScrollAxisDef::Vertical => ScrollAxis::Vertical,
            ScrollAxisDef::Both => ScrollAxis::Both,
        }
    }
}
