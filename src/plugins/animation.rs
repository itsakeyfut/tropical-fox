//! Animation plugin
//!
//! Handles animation system registration and character asset loading.

use bevy::prelude::*;

use crate::config::load_animation_config_optional;
use crate::config::load_characters_config_optional;
use crate::game_state::GameState;
use crate::resources::{CharacterAssets, CharacterTextureAtlas};
use crate::systems::process_animation_events;

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

/// Load character sprite sheets and create texture atlas layouts for all characters
fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading character assets for all characters");

    let mut character_assets = CharacterAssets::new();

    // Load character configuration
    let characters_config = load_characters_config_optional("assets/config/characters.ron");

    if let Some(config) = characters_config {
        // Load assets for all characters defined in characters.ron
        // Iterate through categories (players, bosses, enemies)
        for (category, characters) in &config.characters {
            info!("Loading assets for category: {}", category);

            for (character_id, character_def) in characters {
                let animation_config_path =
                    format!("assets/{}", character_def.animation_config_path);

                let (spritesheet_path, sprite_size, columns, rows) = if let Some(anim_config) =
                    load_animation_config_optional(&animation_config_path)
                {
                    info!(
                        "Loaded animation config for '{}' from {}",
                        character_id, animation_config_path
                    );
                    (
                        anim_config.spritesheet_path,
                        UVec2::new(anim_config.sprite_size.0, anim_config.sprite_size.1),
                        anim_config.columns,
                        anim_config.rows,
                    )
                } else {
                    warn!(
                        "No animation config found for '{}' at {}. Skipping.",
                        character_id, animation_config_path
                    );
                    continue;
                };

                // Load the sprite sheet texture
                let texture = asset_server.load(&spritesheet_path);

                // Create the texture atlas layout
                let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
                let layout_handle = texture_atlas_layouts.add(layout);

                // Insert into character assets
                character_assets.insert(
                    character_id.clone(),
                    CharacterTextureAtlas::new(texture, layout_handle),
                );

                info!(
                    "Character assets loaded for '{}' ({}): {}x{} sprites, {} columns, {} rows",
                    character_id, category, sprite_size.x, sprite_size.y, columns, rows
                );
            }
        }
    } else {
        // Fallback: load only fox with default configuration
        warn!("Could not load characters.ron, loading default fox assets");
        let spritesheet_path = "graphics/characters/players/fox/spritesheets/fox.png";
        let sprite_size = UVec2::new(32, 32);
        let columns = 6;
        let rows = 10;

        let texture = asset_server.load(spritesheet_path);
        let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
        let layout_handle = texture_atlas_layouts.add(layout);

        character_assets.insert("fox", CharacterTextureAtlas::new(texture, layout_handle));

        info!(
            "Fallback character assets loaded for 'fox': {}x{} sprites, {} columns, {} rows",
            sprite_size.x, sprite_size.y, columns, rows
        );
    }

    // Insert the character assets as a resource
    commands.insert_resource(character_assets);
}
