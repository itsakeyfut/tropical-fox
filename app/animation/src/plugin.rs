//! Animation plugin
//!
//! Handles animation system registration and character asset loading.

use bevy::prelude::*;
use tropical_fox_common::{CharacterAssets, CharacterTextureAtlas, GameState};

use crate::systems::{process_animation_events, update_animations};

/// Plugin that manages sprite animations and character assets
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        // Load character assets at startup
        app.add_systems(Startup, load_character_assets);

        // Core animation systems run in FixedUpdate for smooth, frame-rate independent updates
        app.add_systems(
            FixedUpdate,
            update_animations.run_if(in_state(GameState::InGame)),
        );

        // Animation events run in Update
        app.add_systems(
            Update,
            process_animation_events.run_if(in_state(GameState::InGame)),
        );
    }
}

/// Load character sprite sheets and create texture atlas layouts
///
/// Note: This is a simplified version that loads default fox assets.
/// Individual domain plugins (PlayerPlugin, EnemyPlugin) are responsible
/// for loading their own character assets to avoid circular dependencies.
fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading default character assets");

    let mut character_assets = CharacterAssets::new();

    // Load default fox assets
    let spritesheet_path = "graphics/characters/players/fox/spritesheets/fox.png";
    let sprite_size = UVec2::new(32, 32);
    let columns = 6;
    let rows = 10;

    let texture = asset_server.load(spritesheet_path);
    let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    character_assets.insert("fox", CharacterTextureAtlas::new(texture, layout_handle));

    info!(
        "Default character assets loaded for 'fox': {}x{} sprites, {} columns, {} rows",
        sprite_size.x, sprite_size.y, columns, rows
    );

    // Insert the character assets as a resource
    commands.insert_resource(character_assets);
}
