# Tropical Fox - 2Dアクションゲーム設計書

## 目次

1. [プロジェクト概要](#プロジェクト概要)
2. [技術スタック](#技術スタック)
3. [アーキテクチャ設計](#アーキテクチャ設計)
4. [RON設定システム](#ron設定システム)
5. [デバッグモード](#デバッグモード)
6. [実装計画](#実装計画)
7. [ディレクトリ構造](#ディレクトリ構造)

---

## プロジェクト概要

**目的**: 学習を目的としたBevy製2Dアクションゲーム
**アセット**: Sunny Land Collection (CC0ライセンス)
**特徴**:
- 4つのプレイヤーキャラクター（Fox, Squirrel, Bunny Girl, Fiery Imp）
- 30以上の敵タイプ、2つのボス
- 17の異なる環境テーマ
- RONファイルによる柔軟な設定管理
- 安全なデバッグモード

---

## 技術スタック

### 必須依存関係

```toml
[dependencies]
bevy = { version = "0.15", default-features = false, features = [
    "bevy_winit",
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_sprite",
    "bevy_text",
    "bevy_ui",
    "bevy_asset",
    "bevy_scene",
    "bevy_gilrs",  # ゲームパッド対応
    "bevy_audio",
    "vorbis",      # OGGファイルサポート
    "png",
    "x11",         # Linux対応（必要に応じて）
] }

# RON (Rusty Object Notation) - 設定ファイル用
ron = "0.8"
serde = { version = "1.0", features = ["derive"] }

# デバッグ機能（条件付き）
bevy-inspector-egui = { version = "0.28", optional = true }

[features]
default = []
debug_mode = ["bevy-inspector-egui"]
```

### 選択的依存関係（後で追加検討）

- `bevy_ecs_tilemap` - タイルマップシステム（環境構築用）
- `bevy_rapier2d` - 物理エンジン（より複雑な物理が必要な場合）
- `leafwing-input-manager` - 入力管理の抽象化

---

## アーキテクチャ設計

### Bevyプラグイン構造

```
main.rs
├── CorePlugin          - 基本設定、ウィンドウ、リソース初期化
├── AssetsPlugin        - アセットローダー、RON設定読み込み
├── PlayerPlugin        - プレイヤー制御、入力処理
├── AnimationPlugin     - スプライトアニメーション管理
├── EnemyPlugin         - 敵AI、行動パターン
├── CombatPlugin        - ダメージ、衝突判定、戦闘システム
├── LevelPlugin         - レベル読み込み、カメラ制御
├── AudioPlugin         - BGM、効果音管理
└── DebugPlugin         - デバッグ情報表示（debug_mode feature時のみ）
```

### ECSコンポーネント設計

#### 共通コンポーネント

```rust
// src/components/common.rs

/// 体力を持つエンティティ
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// 移動可能なエンティティ
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// 物理パラメータ
#[derive(Component)]
pub struct PhysicsBody {
    pub gravity_scale: f32,
    pub friction: f32,
    pub mass: f32,
}

/// 衝突判定用
#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}

/// ダメージを与える能力
#[derive(Component)]
pub struct DamageDealer {
    pub amount: f32,
    pub knockback: Vec2,
}
```

#### プレイヤー固有

```rust
// src/components/player.rs

#[derive(Component)]
pub struct Player {
    pub character_type: CharacterType,
}

#[derive(Component)]
pub struct PlayerStats {
    pub move_speed: f32,
    pub jump_force: f32,
    pub max_jumps: u8,        // 多段ジャンプ
    pub dash_speed: f32,
    pub invincibility_time: f32,  // 被ダメージ後の無敵時間
}

#[derive(Component)]
pub struct PlayerState {
    pub is_grounded: bool,
    pub facing_right: bool,
    pub current_jumps: u8,
    pub is_invincible: bool,
}

pub enum CharacterType {
    Fox,
    Squirrel,
    BunnyGirl,
    FieryImp,
}
```

#### 敵固有

```rust
// src/components/enemy.rs

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: String,  // "bat", "slug", etc.
}

#[derive(Component)]
pub struct EnemyAI {
    pub behavior: AIBehavior,
    pub detection_range: f32,
    pub attack_range: f32,
    pub patrol_points: Vec<Vec2>,
    pub current_patrol_index: usize,
}

pub enum AIBehavior {
    Patrol,          // 巡回
    Chase,           // プレイヤー追跡
    Flee,            // 逃走
    Stationary,      // 定点
    Flying,          // 飛行パターン
}

#[derive(Component)]
pub struct EnemyStats {
    pub move_speed: f32,
    pub damage: f32,
    pub score_value: u32,
}
```

#### アニメーション

```rust
// src/components/animation.rs

#[derive(Component)]
pub struct AnimationController {
    pub animations: HashMap<String, AnimationClip>,
    pub current_animation: String,
    pub default_animation: String,
}

#[derive(Component)]
pub struct AnimationState {
    pub current_frame: usize,
    pub timer: Timer,
    pub playing: bool,
    pub looping: bool,
}

pub struct AnimationClip {
    pub first_frame: usize,
    pub last_frame: usize,
    pub fps: f32,
}
```

---

## RON設定システム

### 設定ファイル構造

```
assets/config/
├── characters/
│   ├── fox.ron
│   ├── squirrel.ron
│   ├── bunny_girl.ron
│   └── fiery_imp.ron
├── enemies/
│   ├── bat.ron
│   ├── slug.ron
│   └── ... (各敵タイプ)
├── levels/
│   ├── level_01.ron
│   ├── level_02.ron
│   └── ...
└── game_settings.ron
```

### RON設定例

#### キャラクター設定 (`assets/config/characters/fox.ron`)

```ron
(
    // 基本パラメータ
    stats: (
        move_speed: 150.0,        // ピクセル/秒
        jump_force: 300.0,
        max_jumps: 2,
        dash_speed: 250.0,
        invincibility_time: 1.5,  // 秒
    ),

    // 物理パラメータ
    physics: (
        gravity_scale: 1.0,
        friction: 0.8,
        mass: 1.0,
    ),

    // 体力
    health: (
        max: 100.0,
        starting: 100.0,
    ),

    // 衝突判定サイズ
    collider: (
        size: (width: 24.0, height: 28.0),
        offset: (x: 0.0, y: -2.0),
    ),

    // アニメーション設定
    animations: (
        spritesheet_path: "graphics/characters/players/fox/spritesheets/fox.png",
        sprite_size: (width: 32.0, height: 32.0),
        columns: 6,
        rows: 10,

        clips: {
            "idle": (first: 0, last: 3, fps: 8.0),
            "run": (first: 6, last: 11, fps: 12.0),
            "jump": (first: 12, last: 15, fps: 10.0),
            "crouch": (first: 18, last: 20, fps: 6.0),
            "hurt": (first: 24, last: 25, fps: 8.0),
            "dizzy": (first: 48, last: 53, fps: 10.0),
            "roll": (first: 54, last: 57, fps: 15.0),
        },

        default_animation: "idle",
    ),
)
```

#### 敵設定 (`assets/config/enemies/bat.ron`)

```ron
(
    name: "Bat",

    stats: (
        move_speed: 80.0,
        damage: 10.0,
        score_value: 50,
    ),

    health: (
        max: 20.0,
    ),

    ai: (
        behavior: Flying,
        detection_range: 200.0,
        attack_range: 40.0,
        patrol_speed: 60.0,
    ),

    physics: (
        gravity_scale: 0.0,  // 飛行敵は重力無視
        friction: 0.9,
        mass: 0.5,
    ),

    collider: (
        size: (width: 20.0, height: 16.0),
        offset: (x: 0.0, y: 0.0),
    ),

    animations: (
        spritesheet_path: "graphics/characters/enemies/bat/spritesheets/bat.png",
        sprite_size: (width: 32.0, height: 32.0),
        columns: 4,
        rows: 1,

        clips: {
            "fly": (first: 0, last: 3, fps: 10.0),
        },

        default_animation: "fly",
    ),
)
```

#### レベル設定 (`assets/config/levels/level_01.ron`)

```ron
(
    name: "Sunny Land - Level 1",

    environment: (
        tilemap: "graphics/environments/sunny_land/tilesets/tileset.png",
        tile_size: 16.0,
        background_layers: [
            "graphics/environments/sunny_land/layers/back.png",
            "graphics/environments/sunny_land/layers/middle.png",
        ],
        parallax_speeds: [0.3, 0.6],
    ),

    music: "audio/music/sunny_land_theme.ogg",

    player_spawn: (x: 100.0, y: 200.0),

    enemies: [
        (type: "bat", position: (x: 300.0, y: 150.0)),
        (type: "slug", position: (x: 450.0, y: 250.0)),
        (type: "frog", position: (x: 600.0, y: 250.0)),
    ],

    collectibles: [
        (type: "cherry", position: (x: 200.0, y: 180.0)),
        (type: "gem", position: (x: 350.0, y: 120.0)),
        (type: "star", position: (x: 500.0, y: 200.0)),
    ],

    camera: (
        follow_speed: 3.0,
        bounds: (
            min: (x: 0.0, y: 0.0),
            max: (x: 1920.0, y: 1080.0),
        ),
    ),
)
```

#### ゲーム全体設定 (`assets/config/game_settings.ron`)

```ron
(
    window: (
        title: "Tropical Fox",
        width: 1280,
        height: 720,
        resizable: true,
        vsync: true,
    ),

    graphics: (
        pixel_perfect: true,
        scaling_mode: NearestNeighbor,
        target_resolution: (width: 320, height: 180),  // 内部解像度
    ),

    audio: (
        master_volume: 0.7,
        music_volume: 0.6,
        sfx_volume: 0.8,
    ),

    gameplay: (
        gravity: 500.0,
        terminal_velocity: 400.0,
    ),
)
```

### RON読み込みコード例

```rust
// src/config/mod.rs

use serde::Deserialize;
use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterConfig {
    pub stats: CharacterStats,
    pub physics: PhysicsConfig,
    pub health: HealthConfig,
    pub collider: ColliderConfig,
    pub animations: AnimationConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterStats {
    pub move_speed: f32,
    pub jump_force: f32,
    pub max_jumps: u8,
    pub dash_speed: f32,
    pub invincibility_time: f32,
}

// ... 他の構造体定義

/// RONファイルを読み込む汎用関数
pub fn load_config<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file {}: {}", path, e))?;

    ron::from_str(&content)
        .map_err(|e| format!("Failed to parse RON file {}: {}", path, e))
}

/// Bevyリソースとして保存
#[derive(Resource)]
pub struct GameConfigs {
    pub characters: HashMap<String, CharacterConfig>,
    pub enemies: HashMap<String, EnemyConfig>,
    pub levels: HashMap<String, LevelConfig>,
    pub game_settings: GameSettings,
}

impl GameConfigs {
    pub fn load_all() -> Result<Self, String> {
        let mut characters = HashMap::new();
        characters.insert("fox".to_string(), load_config("assets/config/characters/fox.ron")?);
        characters.insert("squirrel".to_string(), load_config("assets/config/characters/squirrel.ron")?);

        // ... 他の設定も同様に読み込み

        Ok(Self {
            characters,
            enemies: HashMap::new(),  // 実装時に追加
            levels: HashMap::new(),
            game_settings: load_config("assets/config/game_settings.ron")?,
        })
    }
}
```

---

## デバッグモード

### 実装方針

デバッグ機能は**Cargoのfeatureフラグ**を使用して、本番ビルドに含まれないようにします。

### Cargo.toml設定

```toml
[features]
default = []
debug_mode = ["bevy-inspector-egui"]

[dependencies.bevy-inspector-egui]
version = "0.28"
optional = true
```

### 使用方法

```bash
# デバッグモード有効でビルド
cargo run --features debug_mode

# デバッグモード無効（通常ビルド）
cargo run

# リリースビルド（デバッグコードは完全に除外される）
cargo build --release
```

### デバッグプラグイン実装

```rust
// src/debug/mod.rs

use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug_mode")]
        {
            // bevy-inspector-eguiを追加
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

            // カスタムデバッグ情報表示
            app.add_systems(Update, (
                display_fps_system,
                display_player_info_system,
                debug_draw_colliders_system,
                debug_spawn_enemy_system,  // キー入力で敵を生成
            ));

            info!("Debug mode enabled");
        }

        #[cfg(not(feature = "debug_mode"))]
        {
            // デバッグモードが無効の場合は何もしない
            info!("Debug mode disabled");
        }
    }
}

#[cfg(feature = "debug_mode")]
fn display_fps_system(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("FPS: {:.1}", value);
            }
        }
    }
}

#[cfg(feature = "debug_mode")]
fn display_player_info_system(
    player_query: Query<(&Transform, &Velocity, &Health), With<Player>>,
    mut query: Query<&mut Text, With<PlayerDebugText>>,
) {
    for (transform, velocity, health) in &player_query {
        for mut text in &mut query {
            text.sections[0].value = format!(
                "Position: ({:.1}, {:.1})\nVelocity: ({:.1}, {:.1})\nHealth: {:.0}/{:.0}",
                transform.translation.x,
                transform.translation.y,
                velocity.x,
                velocity.y,
                health.current,
                health.max
            );
        }
    }
}

#[cfg(feature = "debug_mode")]
fn debug_draw_colliders_system(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Collider)>,
) {
    for (transform, collider) in &query {
        let pos = transform.translation.truncate() + collider.offset;
        gizmos.rect_2d(
            pos,
            0.0,
            collider.size,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}

#[cfg(feature = "debug_mode")]
fn debug_spawn_enemy_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    configs: Res<GameConfigs>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        // プレイヤーの前方に敵を生成
        if let Ok(player_transform) = player_query.get_single() {
            let spawn_pos = player_transform.translation + Vec3::new(100.0, 0.0, 0.0);

            // バットを生成（例）
            if let Some(bat_config) = configs.enemies.get("bat") {
                spawn_enemy(&mut commands, &asset_server, bat_config, spawn_pos);
                info!("Spawned bat at {:?}", spawn_pos);
            }
        }
    }
}

// デバッグUI用マーカーコンポーネント
#[cfg(feature = "debug_mode")]
#[derive(Component)]
struct FpsText;

#[cfg(feature = "debug_mode")]
#[derive(Component)]
struct PlayerDebugText;
```

### デバッグ機能一覧

| 機能 | 説明 | 実装方法 |
|------|------|----------|
| FPS表示 | 画面左上にFPS表示 | `FrameTimeDiagnosticsPlugin` |
| Entity Inspector | 全エンティティの状態をGUIで確認 | `bevy-inspector-egui` |
| Collider可視化 | 当たり判定を緑の枠で表示 | `Gizmos` |
| プレイヤー情報 | 座標、速度、体力をテキスト表示 | カスタムシステム |
| 敵スポーン | F1キーで敵を生成 | キーボード入力 |
| ログ出力 | 詳細なログ（info, debug, trace） | `RUST_LOG=debug` |
| 時間制御 | F2で1/4速再生、F3で通常速度 | `Time<Virtual>` |

---

## 実装計画

### フェーズ1: プロジェクト基礎（1-2日）

**目標**: Bevyの起動とウィンドウ表示

- [ ] `Cargo.toml`に依存関係を追加
- [ ] 基本的なmain.rs構造を作成
- [ ] ウィンドウとカメラを設定
- [ ] アセットフォルダのパス確認

**成果物**: ウィンドウが開くだけの最小限のBevy アプリ

```rust
// main.rsの初期構造例
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
```

---

### フェーズ2: RON設定システム（2-3日）

**目標**: RONファイルから設定を読み込める仕組み

- [ ] `src/config/` モジュールを作成
- [ ] RON構造体定義（Character, Enemy, Level）
- [ ] 設定ファイル読み込み関数
- [ ] サンプルRONファイルを作成（fox.ron, bat.ron）
- [ ] 起動時に設定を読み込んでリソースに保存

**成果物**: 起動時にRONファイルを読み込み、ログに内容を出力

---

### フェーズ3: アセットローダー（2日）

**目標**: スプライトシートを読み込んで表示

- [ ] テクスチャアトラスの読み込み
- [ ] 1つのキャラクター（Fox）を画面中央に表示
- [ ] アニメーションなし（静止画）

**成果物**: Foxの静止画が画面中央に表示される

---

### フェーズ4: アニメーションシステム（3-4日）

**目標**: スプライトアニメーションの実装

- [ ] `AnimationController`コンポーネント
- [ ] `AnimationState`コンポーネント
- [ ] フレーム更新システム（タイマーベース）
- [ ] アニメーション切り替え関数
- [ ] Foxのidleアニメーション再生

**成果物**: Foxがidleアニメーションでループ再生

---

### フェーズ5: プレイヤーコントローラー（4-5日）

**目標**: キーボード入力でキャラクターを動かす

- [ ] 入力処理システム（左右移動、ジャンプ）
- [ ] `Velocity`コンポーネントと物理更新
- [ ] 重力の実装
- [ ] 地面判定（簡易版）
- [ ] アニメーション連動（walk, jump, idle）

**成果物**: キーボードでFoxを左右移動、ジャンプ可能

---

### フェーズ6: 衝突判定システム（3-4日）

**目標**: AABB衝突判定の実装

- [ ] `Collider`コンポーネント
- [ ] 衝突判定関数
- [ ] 地面タイルとの衝突
- [ ] プレイヤーと敵の衝突
- [ ] 衝突イベント発行

**成果物**: プレイヤーが地面に立ち、壁にぶつかる

---

### フェーズ7: 敵システム（4-5日）

**目標**: 敵の配置と基本AI

- [ ] 敵エンティティのスポーン
- [ ] 敵のアニメーション
- [ ] AIシステム（Patrol, Chase）
- [ ] 簡単な巡回パターン
- [ ] プレイヤーとの衝突でダメージ

**成果物**: 巡回する敵がスポーンし、プレイヤーにダメージを与える

---

### フェーズ8: コンバットシステム（3日）

**目標**: 攻撃とダメージ処理

- [ ] プレイヤーの攻撃アクション（ロール、ファイアボール）
- [ ] `DamageDealer`コンポーネント
- [ ] ダメージ判定と体力減少
- [ ] 無敵時間の実装
- [ ] 敵の撃破とスコア

**成果物**: プレイヤーが敵を倒せる

---

### フェーズ9: レベルシステム（5-6日）

**目標**: タイルマップとレベル読み込み

- [ ] タイルマップ表示（`bevy_ecs_tilemap`導入検討）
- [ ] 背景レイヤー（視差効果）
- [ ] レベル設定（RON）からエンティティ配置
- [ ] カメラのプレイヤー追従
- [ ] カメラの境界制限

**成果物**: 完全なレベル1が遊べる

---

### フェーズ10: UI・収集アイテム（3日）

**目標**: ゲームUIと収集要素

- [ ] 体力バー
- [ ] スコア表示
- [ ] 収集アイテム（cherry, gem）
- [ ] アイテム取得エフェクト

**成果物**: UIが表示され、アイテムを集められる

---

### フェーズ11: オーディオ（2日）

**目標**: BGMと効果音

- [ ] BGM再生システム
- [ ] レベルごとのBGM切り替え
- [ ] 効果音（ジャンプ、ダメージ、アイテム取得）

**成果物**: 音楽と効果音が鳴る

---

### フェーズ12: デバッグモード（2日）

**目標**: デバッグ機能の実装

- [ ] `debug_mode` feature設定
- [ ] bevy-inspector-egui統合
- [ ] カスタムデバッグUI
- [ ] コライダー可視化
- [ ] デバッグコマンド（敵スポーン等）

**成果物**: `--features debug_mode`でデバッグ機能が使える

---

### フェーズ13: 改良・調整（継続的）

**目標**: ゲームプレイの磨き上げ

- [ ] パラメータ調整（速度、ジャンプ力、ダメージ）
- [ ] 追加の敵タイプ
- [ ] 追加のレベル
- [ ] エフェクトの追加
- [ ] パフォーマンス最適化

---

## ディレクトリ構造

### 最終的なプロジェクト構造

```
tropical-fox/
├── src/
│   ├── main.rs                      # エントリーポイント
│   ├── config/                      # RON設定関連
│   │   ├── mod.rs
│   │   ├── character.rs
│   │   ├── enemy.rs
│   │   ├── level.rs
│   │   └── game_settings.rs
│   ├── components/                  # ECSコンポーネント
│   │   ├── mod.rs
│   │   ├── common.rs
│   │   ├── player.rs
│   │   ├── enemy.rs
│   │   └── animation.rs
│   ├── systems/                     # Bevyシステム
│   │   ├── mod.rs
│   │   ├── player_control.rs
│   │   ├── physics.rs
│   │   ├── collision.rs
│   │   ├── animation.rs
│   │   ├── enemy_ai.rs
│   │   └── camera.rs
│   ├── plugins/                     # Bevyプラグイン
│   │   ├── mod.rs
│   │   ├── core.rs
│   │   ├── assets.rs
│   │   ├── player.rs
│   │   ├── enemy.rs
│   │   ├── combat.rs
│   │   ├── level.rs
│   │   └── audio.rs
│   ├── resources/                   # Bevyリソース
│   │   ├── mod.rs
│   │   └── game_state.rs
│   ├── events/                      # カスタムイベント
│   │   ├── mod.rs
│   │   ├── combat.rs
│   │   └── collision.rs
│   └── debug/                       # デバッグ機能
│       ├── mod.rs
│       └── plugin.rs
├── assets/
│   ├── config/                      # RON設定ファイル
│   │   ├── game_settings.ron
│   │   ├── characters/
│   │   ├── enemies/
│   │   └── levels/
│   ├── graphics/                    # スプライト（既存）
│   ├── audio/                       # 音楽（既存）
│   └── fonts/                       # フォント（後で追加）
├── memo/
│   ├── design.md                    # この設計書
│   ├── implementation_log.md        # 実装ログ（進捗記録）
│   └── ron_format_guide.md          # RON設定リファレンス
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 注意事項・ベストプラクティス

### RON設定のベストプラクティス

1. **単位を明示**: コメントで`// ピクセル/秒`など単位を書く
2. **デフォルト値を用意**: `Option<T>`で未指定時のフォールバック
3. **バリデーション**: 読み込み後に不正な値をチェック（速度が負など）
4. **ホットリロード**: 開発時に設定変更を自動反映（`bevy-inspector-egui`の機能）

### ECS設計のベストプラクティス

1. **小さなコンポーネント**: 1つのコンポーネントは1つの責任
2. **クエリフィルタ活用**: `With<T>`, `Without<T>`で効率的な検索
3. **イベント駆動**: 直接操作より`EventWriter`/`EventReader`を使う
4. **状態管理**: `State<T>`でゲーム状態（タイトル、プレイ中、ポーズ）を管理

### パフォーマンス考慮

1. **スプライトバッチング**: 同じテクスチャをまとめて描画
2. **カリング**: 画面外のエンティティは描画スキップ
3. **オブジェクトプール**: 敵や弾の頻繁な生成/削除を避ける

### 学習リソース

- [Bevy公式ドキュメント](https://bevyengine.org/learn/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- [RON仕様](https://github.com/ron-rs/ron)

---

## まとめ

この設計書に従って開発を進めることで、以下が実現できます：

1. **柔軟性**: RONファイルでゲームバランスを簡単に調整
2. **保守性**: プラグイン/システム/コンポーネントの明確な分離
3. **学習効果**: Bevy ECSの理解、Rust設計パターンの習得
4. **拡張性**: 新しい敵、レベル、キャラクターを簡単に追加
5. **安全性**: デバッグ機能がリリースビルドに混入しない

**推奨される開発の進め方**:

1. まずフェーズ1-5を実装して**動くキャラクター**を作る
2. フェーズ6-8で**ゲームプレイの核心**（衝突、敵、戦闘）を作る
3. フェーズ9-11で**完全なゲーム体験**にする
4. フェーズ12のデバッグモードは**並行して**実装していくと効率的

各フェーズの終わりには動作確認とテストプレイを行い、次に進む前に品質を確保しましょう。
