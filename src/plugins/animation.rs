//! Animation plugin
//!
//! Handles animation system registration and character asset loading.

use bevy::prelude::*;

use crate::config::{
    SelectedCharacter, load_animation_config_optional, load_characters_config_optional,
};
use crate::game_state::GameState;
use crate::resources::CharacterAssets;
use crate::systems::{player_animation_controller, process_animation_events, update_animations};

/// Plugin that manages sprite animations and character assets
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        // Load character assets at startup
        app.add_systems(Startup, load_character_assets);

        // Animation events run in Update
        // Note: player_animation_controller and update_animations are registered
        // in PlayerPlugin to ensure they run in FixedUpdate for proper synchronization
        app.add_systems(
            Update,
            process_animation_events.run_if(in_state(GameState::InGame)),
        );
    }
}

/// Load character sprite sheets and create texture atlas layouts
fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    selected_character: Option<Res<SelectedCharacter>>,
) {
    // Determine which character to load
    let character_id = selected_character
        .as_ref()
        .map(|sc| sc.character_id.as_str())
        .unwrap_or("fox");

    info!("Loading character assets for: {}", character_id);

    // Load character configuration
    let characters_config = load_characters_config_optional("assets/config/characters.ron");

    // Get animation config path from character definition
    let animation_config_path = if let Some(config) = characters_config {
        if let Ok(character) = config.get_character(character_id) {
            format!("assets/{}", character.animation_config_path)
        } else {
            warn!(
                "Character '{}' not found in characters.ron, using default",
                character_id
            );
            "assets/graphics/characters/players/fox/fox_animations.ron".to_string()
        }
    } else {
        warn!("Could not load characters.ron, using default fox animation config");
        "assets/graphics/characters/players/fox/fox_animations.ron".to_string()
    };

    // Default sprite sheet configuration
    let (spritesheet_path, sprite_size, columns, rows) =
        if let Some(config) = load_animation_config_optional(&animation_config_path) {
            info!("Loaded animation config from {}", animation_config_path);
            (
                config.spritesheet_path,
                UVec2::new(config.sprite_size.0, config.sprite_size.1),
                config.columns,
                config.rows,
            )
        } else {
            info!(
                "No animation config found at {}. Using placeholder configuration.",
                animation_config_path
            );
            // Placeholder configuration for testing
            (
                "graphics/characters/players/fox/spritesheets/fox.png".to_string(),
                UVec2::new(32, 32),
                6,
                10,
            )
        };

    // Load the sprite sheet texture
    let fox_texture = asset_server.load(spritesheet_path);

    // Create the texture atlas layout
    let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
    let fox_layout = texture_atlas_layouts.add(layout);

    // Insert the character assets as a resource
    commands.insert_resource(CharacterAssets::new(fox_layout, fox_texture));

    info!(
        "Character assets loaded for '{}': {}x{} sprites, {} columns, {} rows",
        character_id, sprite_size.x, sprite_size.y, columns, rows
    );
}
