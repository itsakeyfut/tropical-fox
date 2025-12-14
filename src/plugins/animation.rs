//! Animation plugin
//!
//! Handles animation system registration and character asset loading.

use bevy::prelude::*;

use crate::config::load_animation_config_optional;
use crate::config::{
    load_bosses_config_optional, load_enemies_config_optional, load_players_config_optional,
};
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
    let mut loaded_any = false;

    // Load player assets from players.ron
    if let Some(players_config) = load_players_config_optional("assets/config/players.ron") {
        info!("Loading assets for category: players");
        for (player_id, player_def) in &players_config.players {
            if load_character_asset(
                player_id,
                &player_def.animation_config_path,
                "players",
                &asset_server,
                &mut texture_atlas_layouts,
                &mut character_assets,
            ) {
                loaded_any = true;
            }
        }
    }

    // Load boss assets from bosses.ron
    let bosses_config = load_bosses_config_optional("assets/config/bosses.ron");
    if !bosses_config.bosses.is_empty() {
        info!("Loading assets for category: bosses");
        for (boss_id, boss_def) in &bosses_config.bosses {
            if load_character_asset(
                boss_id,
                &boss_def.animation_config_path,
                "bosses",
                &asset_server,
                &mut texture_atlas_layouts,
                &mut character_assets,
            ) {
                loaded_any = true;
            }
        }
    }

    // Load enemy assets from enemies.ron
    let enemies_config = load_enemies_config_optional("assets/config/enemies.ron");
    if !enemies_config.enemies.is_empty() {
        info!("Loading assets for category: enemies");
        for (enemy_id, enemy_def) in &enemies_config.enemies {
            // Skip enemies without animation config path
            if enemy_def.animation_config_path.is_empty() {
                continue;
            }
            if load_character_asset(
                enemy_id,
                &enemy_def.animation_config_path,
                "enemies",
                &asset_server,
                &mut texture_atlas_layouts,
                &mut character_assets,
            ) {
                loaded_any = true;
            }
        }
    }

    // Fallback: load only fox with default configuration if nothing was loaded
    if !loaded_any {
        warn!("Could not load any character configs, loading default fox assets");
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

/// Helper function to load a single character's assets
fn load_character_asset(
    character_id: &str,
    animation_config_path: &str,
    category: &str,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    character_assets: &mut CharacterAssets,
) -> bool {
    let full_path = format!("assets/{}", animation_config_path);

    let (spritesheet_path, sprite_size, columns, rows) =
        if let Some(anim_config) = load_animation_config_optional(&full_path) {
            info!(
                "Loaded animation config for '{}' from {}",
                character_id, full_path
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
                character_id, full_path
            );
            return false;
        };

    // Load the sprite sheet texture
    let texture = asset_server.load(&spritesheet_path);

    // Create the texture atlas layout
    let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    // Insert into character assets
    character_assets.insert(
        character_id.to_string(),
        CharacterTextureAtlas::new(texture, layout_handle),
    );

    info!(
        "Character assets loaded for '{}' ({}): {}x{} sprites, {} columns, {} rows",
        character_id, category, sprite_size.x, sprite_size.y, columns, rows
    );

    true
}
