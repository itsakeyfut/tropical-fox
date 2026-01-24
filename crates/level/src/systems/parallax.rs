//! Parallax background systems
//!
//! Handles spawning and updating parallax background layers

use bevy::prelude::*;

use crate::components::{BackgroundLayer, ParallaxLayer};
use crate::config::{CurrentLevel, load_levels_config_optional};

/// Spawn background layers for the current level
pub fn spawn_background_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    info!(
        "Spawning background layers for level: {}",
        current_level.level_id
    );

    // Load level configuration
    let Some(levels_config) = load_levels_config_optional("assets/config/levels.ron") else {
        warn!("Could not load levels.ron for background spawning");
        return;
    };

    // Get current level config
    let Ok(level_config) = levels_config.get_level(&current_level.level_id) else {
        warn!("Level '{}' not found in config", current_level.level_id);
        return;
    };

    // Spawn each background layer
    for (index, bg_layer) in level_config.background_layers.iter().enumerate() {
        info!(
            "Spawning background layer {}: {} (parallax: {}, z: {})",
            index, bg_layer.sprite_path, bg_layer.parallax_factor, bg_layer.z_order
        );

        // Load the background sprite
        let texture = asset_server.load(&bg_layer.sprite_path);

        // Spawn the background layer entity
        commands.spawn((
            Transform::from_xyz(0.0, 0.0, bg_layer.z_order),
            Visibility::default(),
            Sprite {
                image: texture,
                ..default()
            },
            ParallaxLayer::new(bg_layer.parallax_factor, bg_layer.scroll_speed),
            BackgroundLayer,
            Name::new(format!("Background Layer {}", index)),
        ));
    }

    info!(
        "Spawned {} background layers",
        level_config.background_layers.len()
    );
}

/// Update parallax background positions based on camera movement
pub fn update_parallax(
    camera_query: Query<&Transform, (With<Camera>, Without<ParallaxLayer>)>,
    mut parallax_query: Query<(&mut Transform, &ParallaxLayer), With<BackgroundLayer>>,
    time: Res<Time>,
) {
    // Get camera position (if exists)
    let Some(camera_transform) = camera_query.iter().next() else {
        return;
    };

    let camera_pos = camera_transform.translation;

    // Update each parallax layer
    for (mut transform, parallax) in parallax_query.iter_mut() {
        // Calculate parallax offset based on camera position
        let parallax_offset_x = camera_pos.x * parallax.parallax_factor;
        let parallax_offset_y = camera_pos.y * parallax.parallax_factor;

        // Add automatic scroll
        let scroll_offset_x = parallax.scroll_speed.x * time.elapsed_secs();
        let scroll_offset_y = parallax.scroll_speed.y * time.elapsed_secs();

        // Apply combined offset
        transform.translation.x = parallax.initial_position.x + parallax_offset_x + scroll_offset_x;
        transform.translation.y = parallax.initial_position.y + parallax_offset_y + scroll_offset_y;
    }
}

/// Initialize parallax layer positions
pub fn initialize_parallax_positions(
    mut parallax_query: Query<(&Transform, &mut ParallaxLayer), Added<ParallaxLayer>>,
) {
    for (transform, mut parallax) in parallax_query.iter_mut() {
        parallax.initial_position = transform.translation;
    }
}
