# Tropical Fox - アーキテクチャ設計書

## 概要

このドキュメントでは、**星のカービィ夢の泉**クラスのボリュームに対応できるスケーラブルなゲームアーキテクチャを定義します。

**目標ボリューム感**:
- 複数のワールド（6-8エリア）
- 各ワールドに4-6ステージ + ボス戦
- 合計30-50ステージ
- 複数のキャラクター能力/パワーアップ
- セーブ/ロード機能
- ワールドマップ/ステージセレクト

---

## 目次

1. [システム全体図](#システム全体図)
2. [ゲーム状態管理](#ゲーム状態管理)
3. [プラグインアーキテクチャ](#プラグインアーキテクチャ)
4. [ワールド/ステージ管理](#ワールドステージ管理)
5. [データフロー](#データフロー)
6. [スケーラビリティ戦略](#スケーラビリティ戦略)
7. [セーブデータ設計](#セーブデータ設計)
8. [アセット管理](#アセット管理)
9. [モジュール依存関係](#モジュール依存関係)

---

## システム全体図

```
┌─────────────────────────────────────────────────────────────────┐
│                         Bevy App                                 │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    State Machine                            │ │
│  │  (Title → WorldMap → StagePlay → BossRoom → GameOver)      │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │  Core    │  │  Assets  │  │ Progress │  │  Audio   │       │
│  │  Plugin  │  │  Plugin  │  │  Plugin  │  │  Plugin  │       │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘       │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │  World   │  │  Stage   │  │  Player  │  │  Enemy   │       │
│  │  Map     │  │  Play    │  │  Plugin  │  │  Plugin  │       │
│  │  Plugin  │  │  Plugin  │  └──────────┘  └──────────┘       │
│  └──────────┘  └──────────┘                                     │
│                               ┌──────────┐  ┌──────────┐       │
│  ┌──────────┐  ┌──────────┐  │ Combat   │  │   UI     │       │
│  │Animation │  │ Ability  │  │  Plugin  │  │  Plugin  │       │
│  │  Plugin  │  │  Plugin  │  └──────────┘  └──────────┘       │
│  └──────────┘  └──────────┘                                     │
│                               ┌──────────┐  ┌──────────┐       │
│                               │  Debug   │  │  Input   │       │
│                               │  Plugin  │  │  Plugin  │       │
│                               └──────────┘  └──────────┘       │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                  Shared Resources                           │ │
│  │  - GameConfigs (RON)                                        │ │
│  │  - GameProgress (save data)                                 │ │
│  │  - AssetHandles (textures, audio)                           │ │
│  │  - CurrentStage, CurrentWorld                               │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## ゲーム状態管理

### State階層構造

Bevyの`State<T>`と`SubState<T>`を使った多層状態管理：

```rust
// src/game_state.rs

use bevy::prelude::*;

/// メインゲーム状態
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,        // アセット読み込み中
    Title,          // タイトル画面
    WorldMap,       // ワールドマップ（ステージ選択）
    InGame,         // ゲームプレイ中（SubStateを持つ）
    GameOver,       // ゲームオーバー
    Victory,        // ゲームクリア
}

/// ゲームプレイ中のサブ状態
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    StagePlay,      // 通常のステージプレイ
    BossRoom,       // ボス戦
    Paused,         // ポーズ中
    StageTransition,// ステージ遷移中（フェードイン/アウト）
}

/// プレイヤー状態（さらに細かい制御）
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    Normal,         // 通常状態
    Invincible,     // 無敵状態
    Dead,           // 死亡
    AbilityTransform, // 能力変身中
}
```

### 状態遷移図

```
       ┌─────────┐
       │ Loading │
       └────┬────┘
            │
            ▼
       ┌─────────┐
       │  Title  │◄─────────────────┐
       └────┬────┘                  │
            │                       │
            ▼                       │
      ┌──────────┐                 │
      │WorldMap  │◄───────┐        │
      └────┬─────┘        │        │
           │              │        │
           ▼              │        │
      ┌──────────┐        │        │
      │ InGame   │        │        │
      │  ├─StagePlay      │        │
      │  ├─BossRoom       │        │
      │  ├─Paused         │        │
      │  └─StageTransition│        │
      └────┬─────┘        │        │
           │              │        │
           ├──────────────┘        │
           │                       │
           ├──►┌──────────┐        │
           │   │GameOver  │────────┘
           │   └──────────┘
           │
           └──►┌──────────┐
               │ Victory  │────────┘
               └──────────┘
```

### 状態別のシステム登録

```rust
// src/main.rs

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .init_state::<PlayerState>()

        // Loading状態のシステム
        .add_systems(OnEnter(GameState::Loading), setup_loading)
        .add_systems(Update, check_assets_loaded.run_if(in_state(GameState::Loading)))
        .add_systems(OnExit(GameState::Loading), cleanup_loading)

        // Title状態のシステム
        .add_systems(OnEnter(GameState::Title), setup_title_screen)
        .add_systems(Update, handle_title_input.run_if(in_state(GameState::Title)))

        // WorldMap状態のシステム
        .add_systems(OnEnter(GameState::WorldMap), setup_world_map)
        .add_systems(Update, (
            handle_stage_selection,
            update_world_map_ui,
        ).run_if(in_state(GameState::WorldMap)))

        // InGame状態のシステム（常に動く）
        .add_systems(Update, (
            player_movement,
            enemy_ai,
            physics_system,
            collision_detection,
            animation_system,
        ).run_if(in_state(GameState::InGame)))

        // StagePlay サブ状態のシステム
        .add_systems(Update, (
            check_stage_clear,
            check_player_death,
        ).run_if(in_state(InGameState::StagePlay)))

        // BossRoom サブ状態のシステム
        .add_systems(OnEnter(InGameState::BossRoom), spawn_boss)
        .add_systems(Update, (
            boss_ai_system,
            boss_health_ui,
        ).run_if(in_state(InGameState::BossRoom)))

        // Paused サブ状態のシステム
        .add_systems(OnEnter(InGameState::Paused), show_pause_menu)
        .add_systems(Update, handle_pause_input.run_if(in_state(InGameState::Paused)))
        .add_systems(OnExit(InGameState::Paused), hide_pause_menu)

        .run();
}
```

---

## プラグインアーキテクチャ

### 1. CorePlugin - 基盤システム

**責務**: ゲーム全体の初期化、ウィンドウ設定、共通リソース

```rust
// src/plugins/core.rs

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
            .add_systems(Startup, (
                setup_camera,
                load_game_settings,
            ))
            .add_systems(Update, handle_window_events);
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub window: WindowSettings,
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
}
```

---

### 2. AssetsPlugin - アセット管理

**責務**: テクスチャ、音声、RON設定の読み込みと管理

```rust
// src/plugins/assets.rs

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(OnEnter(GameState::Loading), load_all_assets)
            .add_systems(Update, (
                check_asset_loading,
                handle_asset_events,
            ).run_if(in_state(GameState::Loading)));
    }
}

/// 全てのゲームアセットへのハンドルを保持
#[derive(Resource, Default)]
pub struct GameAssets {
    // キャラクタースプライト
    pub character_sprites: HashMap<String, Handle<Image>>,

    // 敵スプライト
    pub enemy_sprites: HashMap<String, Handle<Image>>,

    // 環境アセット
    pub tileset_handles: HashMap<String, Handle<Image>>,
    pub background_handles: HashMap<String, Handle<Image>>,

    // 音声
    pub music_handles: HashMap<String, Handle<AudioSource>>,
    pub sfx_handles: HashMap<String, Handle<AudioSource>>,

    // RON設定
    pub character_configs: HashMap<String, CharacterConfig>,
    pub enemy_configs: HashMap<String, EnemyConfig>,
    pub stage_configs: HashMap<String, StageConfig>,
    pub world_config: WorldConfig,

    // ロード状態
    pub loading_progress: f32,
}

impl GameAssets {
    /// 指定したワールドのアセットを遅延ロード
    pub fn load_world_assets(&mut self, world_id: u8, asset_server: &AssetServer) {
        // ワールド固有のアセットのみロード（メモリ節約）
    }

    /// 使用していないワールドのアセットをアンロード
    pub fn unload_world_assets(&mut self, world_id: u8) {
        // メモリ管理
    }
}
```

---

### 3. ProgressPlugin - 進行状況管理

**責務**: ゲーム進行状況の追跡、セーブ/ロード

```rust
// src/plugins/progress.rs

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameProgress>()
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<StageClearedEvent>()
            .add_systems(OnEnter(GameState::Title), load_or_create_save)
            .add_systems(Update, (
                handle_save_events,
                handle_load_events,
                handle_stage_cleared,
                auto_save_system,
            ));
    }
}

/// ゲーム進行状況（セーブデータ）
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameProgress {
    // プレイヤー情報
    pub player_name: String,
    pub selected_character: String,
    pub play_time: Duration,

    // ワールド進行状況
    pub worlds: Vec<WorldProgress>,
    pub current_world: u8,
    pub current_stage: u8,

    // 統計
    pub total_score: u32,
    pub total_deaths: u32,
    pub enemies_defeated: HashMap<String, u32>,
    pub items_collected: HashMap<String, u32>,

    // アンロック情報
    pub unlocked_characters: HashSet<String>,
    pub unlocked_abilities: HashSet<String>,

    // セーブメタデータ
    pub last_save_time: SystemTime,
    pub save_slot: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorldProgress {
    pub world_id: u8,
    pub unlocked: bool,
    pub stages: Vec<StageProgress>,
    pub boss_defeated: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StageProgress {
    pub stage_id: u8,
    pub unlocked: bool,
    pub completed: bool,
    pub best_time: Option<Duration>,
    pub best_score: u32,
    pub collectibles_found: u8,
    pub collectibles_total: u8,
}

// イベント定義
#[derive(Event)]
pub struct SaveGameEvent {
    pub slot: u8,
}

#[derive(Event)]
pub struct LoadGameEvent {
    pub slot: u8,
}

#[derive(Event)]
pub struct StageClearedEvent {
    pub world_id: u8,
    pub stage_id: u8,
    pub score: u32,
    pub time: Duration,
    pub collectibles: u8,
}
```

---

### 4. WorldMapPlugin - ワールドマップ

**責務**: ワールドマップUI、ステージ選択

```rust
// src/plugins/world_map.rs

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::WorldMap), setup_world_map)
            .add_systems(Update, (
                handle_world_navigation,
                handle_stage_selection,
                update_map_cursor,
                display_stage_info,
            ).run_if(in_state(GameState::WorldMap)))
            .add_systems(OnExit(GameState::WorldMap), cleanup_world_map);
    }
}

/// ワールドマップ全体の構造
#[derive(Resource)]
pub struct WorldMapData {
    pub worlds: Vec<WorldData>,
    pub cursor_position: WorldMapPosition,
}

#[derive(Clone)]
pub struct WorldData {
    pub id: u8,
    pub name: String,
    pub theme: String,  // "forest", "lava", "winter" etc.
    pub stages: Vec<StageData>,
    pub boss_stage: StageData,
    pub background: String,
    pub music: String,
}

#[derive(Clone)]
pub struct StageData {
    pub id: u8,
    pub name: String,
    pub stage_type: StageType,
    pub difficulty: u8,
    pub map_position: Vec2,  // ワールドマップ上の座標
}

#[derive(Clone, PartialEq)]
pub enum StageType {
    Normal,
    Boss,
    Bonus,
    Secret,
}

pub struct WorldMapPosition {
    pub world_id: u8,
    pub stage_id: u8,
}
```

---

### 5. StagePlayPlugin - ステージプレイ

**責務**: ステージの読み込み、プレイ中のロジック、クリア判定

```rust
// src/plugins/stage_play.rs

pub struct StagePlayPlugin;

impl Plugin for StagePlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StageStartEvent>()
            .add_event::<StageCompleteEvent>()
            .add_event::<StageFailEvent>()
            .add_systems(OnEnter(InGameState::StagePlay), setup_stage)
            .add_systems(Update, (
                check_stage_boundaries,
                check_goal_reached,
                update_stage_timer,
                handle_collectibles,
                handle_checkpoints,
            ).run_if(in_state(InGameState::StagePlay)))
            .add_systems(OnExit(InGameState::StagePlay), cleanup_stage);
    }
}

/// 現在のステージ情報
#[derive(Resource)]
pub struct CurrentStage {
    pub world_id: u8,
    pub stage_id: u8,
    pub config: StageConfig,
    pub elapsed_time: Duration,
    pub score: u32,
    pub collectibles_found: u8,
    pub player_lives: u8,
    pub checkpoint_position: Option<Vec2>,
}

/// ステージ設定（RONから読み込み）
#[derive(Deserialize, Clone)]
pub struct StageConfig {
    pub name: String,
    pub environment: EnvironmentConfig,
    pub music: String,
    pub player_spawn: Vec2,
    pub goal_position: Vec2,

    // エンティティ配置
    pub enemies: Vec<EnemySpawn>,
    pub collectibles: Vec<CollectibleSpawn>,
    pub platforms: Vec<PlatformData>,
    pub hazards: Vec<HazardData>,
    pub checkpoints: Vec<Vec2>,

    // ステージ設定
    pub time_limit: Option<Duration>,
    pub camera_bounds: CameraBounds,
    pub gravity_multiplier: f32,
}

#[derive(Deserialize, Clone)]
pub struct EnemySpawn {
    pub enemy_type: String,
    pub position: Vec2,
    pub patrol_points: Option<Vec<Vec2>>,
    pub facing_right: bool,
}

#[derive(Deserialize, Clone)]
pub struct CollectibleSpawn {
    pub item_type: String,
    pub position: Vec2,
    pub respawns: bool,
}
```

---

### 6. PlayerPlugin - プレイヤーシステム

**責務**: プレイヤー入力、移動、アニメーション

```rust
// src/plugins/player.rs

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerDamageEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_systems(OnEnter(InGameState::StagePlay), spawn_player)
            .add_systems(Update, (
                player_input_system,
                player_movement_system,
                player_jump_system,
                player_animation_system,
                player_collision_response,
                camera_follow_player,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub character_type: String,
    pub lives: u8,
    pub invincibility_timer: Timer,
}

#[derive(Component)]
pub struct PlayerAbilities {
    pub current_ability: Option<String>,
    pub can_double_jump: bool,
    pub can_dash: bool,
    pub can_wall_jump: bool,
}
```

---

### 7. AbilityPlugin - 能力システム

**責務**: コピー能力のような特殊能力の管理

```rust
// src/plugins/ability.rs

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AbilityRegistry>()
            .add_event::<AbilityGainedEvent>()
            .add_event::<AbilityLostEvent>()
            .add_event::<AbilityActivatedEvent>()
            .add_systems(Update, (
                handle_ability_input,
                update_active_abilities,
                ability_visual_effects,
            ).run_if(in_state(GameState::InGame)));
    }
}

/// 能力レジストリ（全ての能力定義）
#[derive(Resource)]
pub struct AbilityRegistry {
    pub abilities: HashMap<String, AbilityDefinition>,
}

#[derive(Clone)]
pub struct AbilityDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub ability_type: AbilityType,
    pub stats_modifier: StatsModifier,
    pub special_actions: Vec<SpecialAction>,
}

#[derive(Clone)]
pub enum AbilityType {
    // 基本能力
    Fire,           // ファイアボール発射
    Ice,            // 敵を凍らせる
    Electric,       // 雷攻撃

    // 移動系
    Wind,           // 滑空
    Speed,          // 高速移動

    // 防御系
    Shield,         // ダメージ軽減
    Heal,           // 体力回復
}

#[derive(Clone)]
pub struct StatsModifier {
    pub move_speed_multiplier: f32,
    pub jump_force_multiplier: f32,
    pub damage_multiplier: f32,
    pub defense_multiplier: f32,
}

#[derive(Clone)]
pub enum SpecialAction {
    Projectile(ProjectileConfig),
    Melee(MeleeConfig),
    Buff(BuffConfig),
}
```

---

### 8. EnemyPlugin - 敵システム

**責務**: 敵のスポーン、AI、行動

```rust
// src/plugins/enemy.rs

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnemyDefeatedEvent>()
            .add_systems(Update, (
                enemy_ai_system,
                enemy_patrol_system,
                enemy_chase_system,
                enemy_attack_system,
                enemy_animation_system,
            ).run_if(in_state(GameState::InGame)));
    }
}
```

---

### 9. CombatPlugin - 戦闘システム

**責務**: ダメージ計算、衝突判定、戦闘エフェクト

```rust
// src/plugins/combat.rs

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_event::<HitEvent>()
            .add_systems(Update, (
                collision_detection_system,
                damage_application_system,
                knockback_system,
                hit_effect_system,
                death_handler_system,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub source: DamageSource,
    pub knockback: Vec2,
}

pub enum DamageSource {
    Enemy(Entity),
    Hazard,
    Projectile(Entity),
    Environment,
}
```

---

### 10. UIPlugin - ユーザーインターフェース

**責務**: HUD、メニュー、ダイアログ

```rust
// src/plugins/ui.rs

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_hud)
            .add_systems(Update, (
                update_health_bar,
                update_score_display,
                update_ability_icon,
                update_timer,
            ).run_if(in_state(GameState::InGame)));
    }
}
```

---

### 11. AudioPlugin - 音響システム

**責務**: BGM、効果音の再生と管理

```rust
// src/plugins/audio.rs

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioController>()
            .add_systems(Update, (
                handle_music_transitions,
                play_sound_effects,
                update_audio_volumes,
            ));
    }
}

#[derive(Resource)]
pub struct AudioController {
    pub current_music: Option<Entity>,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub fade_timer: Option<Timer>,
}
```

---

### 12. InputPlugin - 入力管理

**責務**: キーボード/ゲームパッド入力の抽象化

```rust
// src/plugins/input.rs

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputMap>()
            .add_systems(Update, update_input_state);
    }
}

/// 入力アクション定義
#[derive(Resource)]
pub struct InputMap {
    pub move_left: Vec<InputBinding>,
    pub move_right: Vec<InputBinding>,
    pub jump: Vec<InputBinding>,
    pub attack: Vec<InputBinding>,
    pub ability: Vec<InputBinding>,
    pub pause: Vec<InputBinding>,
}

pub enum InputBinding {
    Keyboard(KeyCode),
    GamepadButton(GamepadButton),
}
```

---

### 13. DebugPlugin - デバッグツール

**責務**: デバッグ情報表示、開発ツール

```rust
// src/plugins/debug.rs

#[cfg(feature = "debug_mode")]
pub struct DebugPlugin;

#[cfg(feature = "debug_mode")]
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
            .add_systems(Update, (
                debug_draw_colliders,
                debug_info_display,
                debug_teleport,
                debug_spawn_enemy,
                debug_unlock_all,
            ));
    }
}
```

---

## ワールド/ステージ管理

### ワールド構造

```
Game
├── World 1: Sunny Land (森林エリア)
│   ├── Stage 1-1: Forest Path
│   ├── Stage 1-2: Tree Tops
│   ├── Stage 1-3: Hidden Cave
│   ├── Stage 1-4: Waterfall
│   └── Boss: Forest Guardian
├── World 2: Lava Zone (火山エリア)
│   ├── Stage 2-1: Volcanic Plains
│   ├── Stage 2-2: Magma Cavern
│   ├── Stage 2-3: Fire Temple
│   ├── Stage 2-4: Dragon's Lair
│   └── Boss: Volcano Dragon
├── World 3: Winter Kingdom (雪原エリア)
│   ├── Stage 3-1: Snowy Hills
│   ├── Stage 3-2: Ice Palace
│   ├── Stage 3-3: Frozen Lake
│   ├── Stage 3-4: Blizzard Peak
│   └── Boss: Ice Colossus
├── World 4: Mushroom Forest (魔法の森エリア)
│   ├── ...
├── World 5: Shadow Realm (闇の世界エリア)
│   ├── ...
├── World 6: Sky Fortress (空中要塞エリア)
│   ├── ...
└── Final World: Rainbow Resort (最終エリア)
    ├── Stage 7-1: Rainbow Road
    ├── Stage 7-2: Dream Palace
    ├── Stage 7-3: Nightmare Gauntlet
    └── Final Boss: Ultimate Evil
```

### RON設定構造

```
assets/config/
├── game_settings.ron
├── worlds/
│   ├── world_config.ron          # 全ワールドの定義
│   ├── world_01_sunny_land.ron
│   ├── world_02_lava_zone.ron
│   ├── world_03_winter_kingdom.ron
│   └── ...
├── stages/
│   ├── stage_1_1.ron
│   ├── stage_1_2.ron
│   ├── stage_1_3.ron
│   └── ...
├── characters/
│   ├── fox.ron
│   ├── squirrel.ron
│   └── ...
├── enemies/
│   ├── bat.ron
│   ├── slug.ron
│   └── ...
└── abilities/
    ├── fire.ron
    ├── ice.ron
    └── ...
```

### ワールド設定例

```ron
// assets/config/worlds/world_config.ron
(
    worlds: [
        (
            id: 1,
            name: "Sunny Land",
            theme: "forest",
            unlock_condition: Always,
            background_music: "audio/music/sunny_land_theme.ogg",
            stage_count: 4,
            has_boss: true,
        ),
        (
            id: 2,
            name: "Lava Zone",
            theme: "lava",
            unlock_condition: CompleteWorld(1),
            background_music: "audio/music/lava_theme.ogg",
            stage_count: 4,
            has_boss: true,
        ),
        // ... 他のワールド
    ],
)
```

---

## データフロー

### ステージ開始時のフロー

```
1. WorldMapで選択
   │
   ▼
2. StageStartEvent発火
   │
   ▼
3. GameState → InGame
   InGameState → StageTransition
   │
   ▼
4. AssetsPlugin: ステージアセット読み込み
   - 環境テクスチャ
   - 敵スプライト
   - BGM
   │
   ▼
5. StagePlayPlugin: ステージセットアップ
   - タイルマップ生成
   - プレイヤースポーン
   - 敵スポーン
   - 収集アイテム配置
   │
   ▼
6. InGameState → StagePlay
   │
   ▼
7. ゲームループ開始
   - 入力処理
   - 物理更新
   - AI更新
   - 描画
```

### ステージクリア時のフロー

```
1. ゴールに到達
   │
   ▼
2. StageCompleteEvent発火
   │
   ▼
3. スコア計算
   - タイム
   - 収集アイテム
   - 敵撃破数
   - ノーダメージボーナス
   │
   ▼
4. GameProgress更新
   - ステージクリアフラグ
   - ベストスコア更新
   - 次ステージアンロック
   │
   ▼
5. SaveGameEvent発火（オートセーブ）
   │
   ▼
6. InGameState → StageTransition
   │
   ▼
7. リザルト画面表示
   │
   ▼
8. GameState → WorldMap
   │
   ▼
9. 次のステージが選択可能に
```

---

## スケーラビリティ戦略

### 1. アセットの遅延ロード

**問題**: 全アセットをメモリに保持すると容量不足

**解決策**:
```rust
// 必要なワールドのアセットのみロード
impl GameAssets {
    pub fn load_world(&mut self, world_id: u8, asset_server: &AssetServer) {
        // 現在のワールド+前後1ワールドのみロード
        let worlds_to_load = vec![
            world_id.saturating_sub(1),
            world_id,
            world_id + 1,
        ];

        for id in worlds_to_load {
            if !self.loaded_worlds.contains(&id) {
                self.load_world_assets(id, asset_server);
            }
        }

        // 使わないワールドをアンロード
        self.unload_distant_worlds(world_id);
    }
}
```

### 2. エンティティのストリーミング

**問題**: 大きなステージで全エンティティを生成するとパフォーマンス低下

**解決策**:
```rust
// カメラ範囲外のエンティティは非アクティブ化
fn stream_entities(
    camera_query: Query<&Transform, With<Camera>>,
    mut entity_query: Query<(&Transform, &mut Visibility, &StreamableEntity)>,
) {
    let camera_pos = camera_query.single().translation.truncate();
    let stream_distance = 800.0; // カメラから800px以内のみアクティブ

    for (transform, mut visibility, _) in &mut entity_query {
        let distance = camera_pos.distance(transform.translation.truncate());

        *visibility = if distance < stream_distance {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[derive(Component)]
struct StreamableEntity;
```

### 3. システムのセット分割

**問題**: 全システムが毎フレーム動くと重い

**解決策**:
```rust
// システムセットで優先度管理
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySet {
    Input,      // 最優先
    Logic,      // ゲームロジック
    Physics,    // 物理演算
    Animation,  // アニメーション
    Rendering,  // 描画準備（最後）
}

app.configure_sets(
    Update,
    (
        GameplaySet::Input,
        GameplaySet::Logic,
        GameplaySet::Physics,
        GameplaySet::Animation,
        GameplaySet::Rendering,
    ).chain() // この順序で実行
);

app.add_systems(
    Update,
    player_input_system.in_set(GameplaySet::Input)
);
```

### 4. RON設定のキャッシュ

**問題**: 毎回RONファイルを読むとディスクI/Oが遅い

**解決策**:
```rust
#[derive(Resource)]
pub struct ConfigCache {
    pub enemies: HashMap<String, Arc<EnemyConfig>>,
    pub stages: HashMap<String, Arc<StageConfig>>,
}

// Arcで共有参照を持つことでメモリ効率化
```

### 5. マルチスレッド対応

**問題**: 大量のエンティティで処理が重い

**解決策**: Bevyの並列システムを活用
```rust
// クエリを自動で並列処理
app.add_systems(Update, (
    enemy_ai_system,      // これらは並列実行可能
    projectile_system,    // （互いに独立したクエリ）
    particle_system,
).chain()); // chainしない限り並列実行される
```

---

## セーブデータ設計

### ファイル構造

```
saves/
├── slot_1.ron
├── slot_2.ron
├── slot_3.ron
└── autosave.ron
```

### セーブデータ例

```ron
// saves/slot_1.ron
(
    version: 1,
    player_name: "Player1",
    selected_character: "fox",
    play_time_seconds: 7234,  // 約2時間

    // 進行状況
    current_world: 3,
    current_stage: 2,

    worlds: [
        (
            world_id: 1,
            unlocked: true,
            stages: [
                (stage_id: 1, unlocked: true, completed: true, best_time_secs: 145, best_score: 5000, collectibles_found: 5, collectibles_total: 5),
                (stage_id: 2, unlocked: true, completed: true, best_time_secs: 203, best_score: 4200, collectibles_found: 4, collectibles_total: 5),
                (stage_id: 3, unlocked: true, completed: true, best_time_secs: 312, best_score: 3800, collectibles_found: 5, collectibles_total: 5),
                (stage_id: 4, unlocked: true, completed: true, best_time_secs: 189, best_score: 4500, collectibles_found: 3, collectibles_total: 5),
            ],
            boss_defeated: true,
        ),
        (
            world_id: 2,
            unlocked: true,
            stages: [
                (stage_id: 1, unlocked: true, completed: true, best_time_secs: 167, best_score: 4800, collectibles_found: 5, collectibles_total: 5),
                (stage_id: 2, unlocked: true, completed: true, best_time_secs: 234, best_score: 3900, collectibles_found: 3, collectibles_total: 5),
                (stage_id: 3, unlocked: true, completed: true, best_time_secs: 289, best_score: 4100, collectibles_found: 4, collectibles_total: 5),
                (stage_id: 4, unlocked: true, completed: false, best_time_secs: 0, best_score: 0, collectibles_found: 0, collectibles_total: 5),
            ],
            boss_defeated: false,
        ),
        (
            world_id: 3,
            unlocked: true,
            stages: [
                (stage_id: 1, unlocked: true, completed: true, best_time_secs: 156, best_score: 4700, collectibles_found: 5, collectibles_total: 5),
                (stage_id: 2, unlocked: true, completed: false, best_time_secs: 0, best_score: 0, collectibles_found: 0, collectibles_total: 5),
                (stage_id: 3, unlocked: false, completed: false, best_time_secs: 0, best_score: 0, collectibles_found: 0, collectibles_total: 5),
                (stage_id: 4, unlocked: false, completed: false, best_time_secs: 0, best_score: 0, collectibles_found: 0, collectibles_total: 5),
            ],
            boss_defeated: false,
        ),
    ],

    // 統計
    total_score: 34000,
    total_deaths: 47,
    enemies_defeated: {
        "bat": 23,
        "slug": 18,
        "frog": 15,
        "lizard": 12,
    },
    items_collected: {
        "cherry": 56,
        "gem": 23,
        "star": 8,
    },

    // アンロック
    unlocked_characters: ["fox", "squirrel"],
    unlocked_abilities: ["fire", "ice"],

    // メタデータ
    last_save_timestamp: 1733875200,  // UNIX timestamp
    save_slot: 1,
)
```

### セーブ/ロード実装

```rust
// src/systems/save_load.rs

use std::path::PathBuf;

pub fn save_game(progress: &GameProgress, slot: u8) -> Result<(), String> {
    let save_dir = get_save_directory();
    std::fs::create_dir_all(&save_dir)
        .map_err(|e| format!("Failed to create save directory: {}", e))?;

    let file_path = save_dir.join(format!("slot_{}.ron", slot));

    let ron_string = ron::ser::to_string_pretty(progress, Default::default())
        .map_err(|e| format!("Failed to serialize save data: {}", e))?;

    std::fs::write(&file_path, ron_string)
        .map_err(|e| format!("Failed to write save file: {}", e))?;

    info!("Game saved to slot {}", slot);
    Ok(())
}

pub fn load_game(slot: u8) -> Result<GameProgress, String> {
    let file_path = get_save_directory().join(format!("slot_{}.ron", slot));

    if !file_path.exists() {
        return Err(format!("Save file not found: slot {}", slot));
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read save file: {}", e))?;

    let progress: GameProgress = ron::from_str(&content)
        .map_err(|e| format!("Failed to deserialize save data: {}", e))?;

    info!("Game loaded from slot {}", slot);
    Ok(progress)
}

fn get_save_directory() -> PathBuf {
    // プラットフォーム別のセーブディレクトリ
    #[cfg(target_os = "windows")]
    {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("TropicalFox")
            .join("saves")
    }

    #[cfg(target_os = "macos")]
    {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("TropicalFox")
            .join("saves")
    }

    #[cfg(target_os = "linux")]
    {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tropical-fox")
            .join("saves")
    }
}
```

---

## アセット管理

### ディレクトリ構造の最適化

```
assets/
├── config/
│   └── (RONファイル)
├── graphics/
│   ├── characters/
│   ├── environments/
│   │   ├── world_01/  # ワールド別に整理
│   │   ├── world_02/
│   │   └── world_03/
│   └── ui/
├── audio/
│   ├── music/
│   │   ├── world_01/
│   │   ├── world_02/
│   │   └── boss/
│   └── sfx/
└── fonts/
```

### アセットバンドル（将来的な最適化）

```rust
// 複数のアセットを1つのファイルにパック
// → ロード時間短縮、ファイル数削減

#[derive(Asset, TypePath)]
pub struct WorldBundle {
    pub tileset: Handle<Image>,
    pub backgrounds: Vec<Handle<Image>>,
    pub enemy_sprites: Vec<Handle<Image>>,
    pub music: Handle<AudioSource>,
}
```

---

## モジュール依存関係

```
main.rs
 │
 ├─► plugins/
 │    ├─► core.rs (依存なし)
 │    ├─► assets.rs (core)
 │    ├─► progress.rs (core)
 │    ├─► input.rs (core)
 │    ├─► audio.rs (core, assets)
 │    ├─► world_map.rs (core, assets, progress, ui)
 │    ├─► stage_play.rs (core, assets, progress, player, enemy)
 │    ├─► player.rs (core, input, animation, ability)
 │    ├─► enemy.rs (core, animation)
 │    ├─► ability.rs (core, player)
 │    ├─► combat.rs (core, player, enemy)
 │    ├─► animation.rs (core)
 │    ├─► ui.rs (core)
 │    └─► debug.rs (全てに依存)
 │
 ├─► components/ (依存なし - データ定義のみ)
 ├─► systems/ (components, resources)
 ├─► resources/ (依存なし)
 ├─► events/ (依存なし)
 └─► config/ (依存なし - serde/ron)
```

**依存関係のルール**:
1. `core`, `components`, `resources`, `events`は他に依存しない
2. プラグインは循環依存を避ける
3. デバッグ機能は他のモジュールを監視するのみ（逆依存なし）

---

## パフォーマンス目標

### ターゲット仕様

| 項目 | 目標 |
|------|------|
| FPS | 60fps (16.67ms/frame) |
| メモリ使用量 | < 500MB |
| ロード時間（ステージ） | < 2秒 |
| セーブ時間 | < 100ms |
| 同時表示エンティティ | 200-300 |

### 最適化ポイント

1. **スプライトバッチング**: 同じテクスチャをまとめて描画
2. **カリング**: 画面外は描画スキップ
3. **オブジェクトプール**: 頻繁に生成/削除するエンティティを再利用
4. **LOD (Level of Detail)**: 遠い敵は簡易描画
5. **並列処理**: 独立したシステムは並列実行

---

## まとめ

このアーキテクチャは以下を実現します：

### スケーラビリティ
- **ワールド追加**: RONファイルを追加するだけ
- **ステージ追加**: 新しいステージ設定ファイルを作成
- **敵追加**: 敵設定ファイル + スプライトを追加
- **能力追加**: 能力定義をレジストリに登録

### 保守性
- **プラグイン分離**: 機能ごとに独立
- **状態管理**: 明確な状態遷移
- **データ駆動**: コード変更なしでバランス調整

### 拡張性
- **新機能追加**: 新しいプラグインを追加
- **マルチプレイ対応**: NetworkPluginを追加（将来）
- **MOD対応**: 外部RONファイル読み込み（将来）

**次のステップ**:
1. 基本プラグイン（Core, Assets, Progress）の実装
2. 簡単なプロトタイプステージで動作確認
3. 段階的に機能を追加
