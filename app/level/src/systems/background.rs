use bevy::prelude::*;
use tropical_fox_hot_asset::HotAssetHandle;

use crate::components::ParallaxLayer;
use crate::config::LevelMetadataConfig;
use crate::resources::CurrentLevel;

/// Marker component for background layers to allow cleanup
#[derive(Component)]
pub struct BackgroundLayer;

/// Spawns background layers for the current level
pub fn spawn_backgrounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    metadata_handle: Res<HotAssetHandle<LevelMetadataConfig>>,
    metadata: Res<Assets<LevelMetadataConfig>>,
    current_level: Res<CurrentLevel>,
    existing_backgrounds: Query<Entity, With<BackgroundLayer>>,
) {
    // Skip if no level is set or metadata not loaded yet
    if current_level.name.is_empty() {
        return;
    }

    let Some(config) = metadata.get(&metadata_handle.0) else {
        return;
    };

    let Some(level_meta) = config.levels.get(&current_level.name) else {
        warn!("No metadata found for level: {}", current_level.name);
        return;
    };

    // Clean up existing backgrounds
    for entity in existing_backgrounds.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new background layers
    for (i, bg_def) in level_meta.backgrounds.iter().enumerate() {
        info!(
            "Spawning background layer {}: {} (parallax: {}, z: {})",
            i, bg_def.texture_path, bg_def.parallax_speed, bg_def.z_index
        );

        commands.spawn((
            Sprite::from_image(asset_server.load(&bg_def.texture_path)),
            Transform::from_xyz(0.0, 0.0, bg_def.z_index),
            ParallaxLayer {
                scroll_speed: bg_def.parallax_speed,
                scroll_axis: bg_def.scroll_axis.into(),
            },
            BackgroundLayer,
            Name::new(format!("Background Layer {}", i)),
        ));
    }
}

/// Updates parallax background positions based on camera movement
pub fn update_parallax(
    camera_query: Query<&Transform, (With<Camera2d>, Changed<Transform>)>,
    mut parallax_query: Query<(&mut Transform, &ParallaxLayer), Without<Camera2d>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for (mut bg_transform, parallax) in parallax_query.iter_mut() {
        use crate::components::ScrollAxis;
        match parallax.scroll_axis {
            ScrollAxis::Horizontal => {
                bg_transform.translation.x = camera_transform.translation.x * parallax.scroll_speed;
            }
            ScrollAxis::Vertical => {
                bg_transform.translation.y = camera_transform.translation.y * parallax.scroll_speed;
            }
            ScrollAxis::Both => {
                bg_transform.translation.x = camera_transform.translation.x * parallax.scroll_speed;
                bg_transform.translation.y = camera_transform.translation.y * parallax.scroll_speed;
            }
        }
    }
}
