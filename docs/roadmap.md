# Tropical Fox - 実装ロードマップ

## 概要

このドキュメントは、最新の2Dアクションゲームの機能を網羅した長期学習向けの詳細な実装計画です。Celeste、Hollow Knight、Ori、Cuphead、Dead Cellsなどの最新タイトルから学んだベストプラクティスを実装していきます。

**目標**: 学びながら、現代的で洗練された2Dアクションゲームを作る

---

## 進捗トラッカー

| Phase | 名称 | 状態 | 重要度 | 難易度 |
|-------|------|------|--------|--------|
| 0 | 環境構築 | 🔵 TODO | ⭐⭐⭐ | ⭐ |
| 1 | 基礎システム | 🔵 TODO | ⭐⭐⭐ | ⭐ |
| 2 | プレイヤー基本動作 | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 3 | 先進的な移動システム | 🔵 TODO | ⭐⭐⭐ | ⭐⭐⭐ |
| 4 | アニメーションシステム | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 5 | 戦闘基礎 | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 6 | 敵システムI | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 7 | レベルシステム基礎 | 🔵 TODO | ⭐⭐⭐ | ⭐⭐⭐ |
| 8 | カメラシステムI | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 9 | ビジュアルエフェクトI | 🔵 TODO | ⭐⭐ | ⭐⭐ |
| 10 | UIシステムI | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 11 | 能力システム | 🔵 TODO | ⭐⭐⭐ | ⭐⭐⭐ |
| 12 | 戦闘システムII | 🔵 TODO | ⭐⭐ | ⭐⭐⭐ |
| 13 | 敵システムII | 🔵 TODO | ⭐⭐ | ⭐⭐⭐ |
| 14 | ボスシステム | 🔵 TODO | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| 15 | ワールドマップ | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 16 | 進行管理 | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 17 | UIシステムII | 🔵 TODO | ⭐⭐ | ⭐⭐ |
| 18 | オーディオシステムI | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 19 | ビジュアルエフェクトII | 🔵 TODO | ⭐⭐ | ⭐⭐⭐⭐ |
| 20 | レベルギミック | 🔵 TODO | ⭐⭐ | ⭐⭐ |
| 21 | カメラシステムII | 🔵 TODO | ⭐⭐ | ⭐⭐⭐ |
| 22 | 収集要素 | 🔵 TODO | ⭐⭐ | ⭐⭐ |
| 23 | オーディオシステムII | 🔵 TODO | ⭐ | ⭐⭐⭐ |
| 24 | パフォーマンス最適化I | 🔵 TODO | ⭐⭐⭐ | ⭐⭐⭐ |
| 25 | アクセシビリティ | 🔵 TODO | ⭐⭐ | ⭐⭐ |
| 26 | チュートリアルシステム | 🔵 TODO | ⭐⭐ | ⭐⭐ |
| 27 | カットシーン/ダイアログ | 🔵 TODO | ⭐ | ⭐⭐⭐ |
| 28 | タイムアタック | 🔵 TODO | ⭐ | ⭐⭐ |
| 29 | リプレイシステム | 🔵 TODO | ⭐ | ⭐⭐⭐⭐ |
| 30 | パフォーマンス最適化II | 🔵 TODO | ⭐⭐ | ⭐⭐⭐ |
| 31 | ポリッシュ | 🔵 TODO | ⭐⭐⭐ | ⭐⭐ |
| 32 | オンライン機能 (オプション) | 🔵 TODO | ⭐ | ⭐⭐⭐⭐⭐ |

**凡例**:
- 🔵 TODO / 🟡 進行中 / 🟢 完了 / ⚫ スキップ
- 重要度: ⭐(低) ~ ⭐⭐⭐(高)
- 難易度: ⭐(簡単) ~ ⭐⭐⭐⭐⭐(非常に難しい)

---

## Phase 0: 環境構築（予想時間: 1-2日）

### 目標
開発環境を整え、プロジェクトの基盤を作る

### タスク

#### 0.1 依存関係のセットアップ
- [ ] Cargo.tomlにBevy 0.15を追加
- [ ] RON、serdeを追加
- [ ] bevy-inspector-eguiを追加（debug_mode feature）
- [ ] その他必要な依存関係

```toml
[dependencies]
bevy = { version = "0.15", default-features = false, features = [
    "bevy_winit", "bevy_render", "bevy_core_pipeline",
    "bevy_sprite", "bevy_text", "bevy_ui",
    "bevy_asset", "bevy_scene", "bevy_audio",
    "bevy_gilrs", "vorbis", "png", "x11"
] }
ron = "0.8"
serde = { version = "1.0", features = ["derive"] }

[features]
default = []
debug_mode = ["bevy-inspector-egui"]

[dependencies.bevy-inspector-egui]
version = "0.28"
optional = true
```

#### 0.2 ディレクトリ構造の作成
- [ ] `src/`配下のモジュール構造を作成
- [ ] `assets/config/`ディレクトリを作成
- [ ] `docs/`配下のドキュメント整備

```
src/
├── main.rs
├── game_state.rs
├── components/
│   └── mod.rs
├── systems/
│   └── mod.rs
├── plugins/
│   ├── mod.rs
│   └── core.rs
├── resources/
│   └── mod.rs
├── events/
│   └── mod.rs
├── config/
│   └── mod.rs
└── debug/
    └── mod.rs
```

#### 0.3 基本的なmain.rsの作成
- [ ] Bevyアプリの初期化
- [ ] Stateの定義
- [ ] 基本プラグインの登録

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
```

#### 0.4 バージョン管理の設定
- [ ] .gitignoreの確認
- [ ] 初期コミット

### 成果物
- ビルドが通るBevy プロジェクト
- 空のウィンドウが表示される

### 学習リソース
- [Bevy公式ドキュメント](https://bevyengine.org/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)

---

## Phase 1: 基礎システム（予想時間: 2-3日）

### 目標
ゲームの基盤となるシステムを構築

### タスク

#### 1.1 ゲーム状態管理
- [ ] `GameState` Enumの定義
- [ ] `InGameState` SubStateの定義
- [ ] 状態遷移システムの実装

```rust
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    WorldMap,
    InGame,
    Paused,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    StagePlay,
    BossRoom,
    StageTransition,
}
```

#### 1.2 CorePluginの実装
- [ ] カメラのセットアップ
- [ ] ウィンドウ設定の読み込み
- [ ] グローバルリソースの初期化

#### 1.3 基本的な物理システム
- [ ] 重力の実装
- [ ] Velocityコンポーネント
- [ ] 基本的な位置更新システム

```rust
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Gravity {
    pub scale: f32,
}

fn apply_gravity(
    time: Res<Time>,
    game_settings: Res<GameSettings>,
    mut query: Query<(&mut Velocity, &Gravity)>,
) {
    let gravity = game_settings.gameplay.gravity;
    for (mut velocity, grav) in &mut query {
        velocity.y -= gravity * grav.scale * time.delta_seconds();
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
```

#### 1.4 RON設定の読み込み
- [ ] GameSettings構造体の定義
- [ ] RON読み込み関数
- [ ] game_settings.ronファイルの作成

### 成果物
- 状態管理システムが動作
- 基本的な物理演算が動作

---

## Phase 2: プレイヤー基本動作（予想時間: 3-4日）

### 目標
プレイヤーを動かせるようにする

### タスク

#### 2.1 プレイヤーエンティティのスポーン
- [ ] Playerコンポーネント
- [ ] プレイヤースプライトの表示
- [ ] 初期位置の設定

#### 2.2 左右移動の実装
- [ ] キーボード入力の処理
- [ ] 水平方向の速度制御
- [ ] 向き（facing）の管理

```rust
#[derive(Component)]
pub struct Player {
    pub facing_right: bool,
}

#[derive(Component)]
pub struct PlayerStats {
    pub move_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

fn player_horizontal_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &PlayerStats)>,
    time: Res<Time>,
) {
    for (mut velocity, mut player, stats) in &mut query {
        let mut input = 0.0;

        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            input -= 1.0;
            player.facing_right = false;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            input += 1.0;
            player.facing_right = true;
        }

        let target_velocity = input * stats.move_speed;

        // スムーズな加速/減速
        if input != 0.0 {
            velocity.x = velocity.x.lerp(target_velocity, stats.acceleration * time.delta_seconds());
        } else {
            velocity.x = velocity.x.lerp(0.0, stats.deceleration * time.delta_seconds());
        }
    }
}
```

#### 2.3 基本的なジャンプ
- [ ] ジャンプ入力の処理
- [ ] 地面判定（簡易版）
- [ ] ジャンプ力の適用

```rust
#[derive(Component)]
pub struct GroundDetection {
    pub is_grounded: bool,
    pub ground_check_distance: f32,
}

fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection, &PlayerStats)>,
) {
    for (mut velocity, ground, stats) in &mut query {
        if ground.is_grounded && keyboard.just_pressed(KeyCode::Space) {
            velocity.y = stats.jump_force;
        }
    }
}
```

#### 2.4 基本的な衝突判定
- [ ] Colliderコンポーネント
- [ ] AABB衝突判定
- [ ] 地面との衝突処理

```rust
#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct Ground;

fn ground_collision(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut GroundDetection, &Collider), With<Player>>,
    ground_query: Query<(&Transform, &Collider), (With<Ground>, Without<Player>)>,
) {
    for (mut player_transform, mut velocity, mut ground_detection, player_collider) in &mut player_query {
        ground_detection.is_grounded = false;

        for (ground_transform, ground_collider) in &ground_query {
            // AABB衝突判定
            let player_pos = player_transform.translation.truncate();
            let ground_pos = ground_transform.translation.truncate();

            // 衝突していたら押し戻す
            if check_aabb_collision(player_pos, player_collider, ground_pos, ground_collider) {
                // 上から衝突した場合
                if velocity.y < 0.0 {
                    player_transform.translation.y = ground_transform.translation.y
                        + ground_collider.size.y / 2.0
                        + player_collider.size.y / 2.0;
                    velocity.y = 0.0;
                    ground_detection.is_grounded = true;
                }
            }
        }
    }
}
```

#### 2.5 スプライトの反転
- [ ] 向きに応じたスプライト反転

```rust
fn flip_sprite_by_facing(
    mut query: Query<(&Player, &mut Sprite)>,
) {
    for (player, mut sprite) in &mut query {
        sprite.flip_x = !player.facing_right;
    }
}
```

### 成果物
- キーボードでキャラクターを左右に動かせる
- スペースキーでジャンプできる
- 地面に着地できる

---

## Phase 3: 先進的な移動システム（予想時間: 4-5日）

### 目標
現代的な2Dアクションゲームの"気持ちいい"動きを実装

### タスク

#### 3.1 コヨーテタイム（Coyote Time）
崖から落ちた直後でもジャンプ可能にする

- [ ] コヨーテタイマーの実装
- [ ] 地面を離れた瞬間の検知
- [ ] タイマー中のジャンプ許可

```rust
#[derive(Component)]
pub struct CoyoteTime {
    pub timer: Timer,
    pub was_grounded: bool,
}

fn update_coyote_time(
    time: Res<Time>,
    mut query: Query<(&GroundDetection, &mut CoyoteTime)>,
) {
    for (ground, mut coyote) in &mut query {
        if ground.is_grounded {
            coyote.timer.reset();
            coyote.was_grounded = true;
        } else if coyote.was_grounded {
            coyote.timer.tick(time.delta());
        }
    }
}

fn player_jump_with_coyote(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection, &CoyoteTime, &PlayerStats)>,
) {
    for (mut velocity, ground, coyote, stats) in &mut query {
        let can_jump = ground.is_grounded || !coyote.timer.finished();

        if can_jump && keyboard.just_pressed(KeyCode::Space) {
            velocity.y = stats.jump_force;
        }
    }
}
```

#### 3.2 ジャンプバッファリング
ジャンプボタンを少し早く押してもジャンプが発動

- [ ] ジャンプ入力バッファの実装
- [ ] バッファ時間内の入力記憶
- [ ] 着地時の自動ジャンプ

```rust
#[derive(Component)]
pub struct JumpBuffer {
    pub timer: Timer,
    pub buffered: bool,
}

fn buffer_jump_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut JumpBuffer>,
) {
    for mut buffer in &mut query {
        if keyboard.just_pressed(KeyCode::Space) {
            buffer.buffered = true;
            buffer.timer.reset();
        }

        if buffer.buffered {
            buffer.timer.tick(time.delta());
            if buffer.timer.finished() {
                buffer.buffered = false;
            }
        }
    }
}

fn consume_jump_buffer(
    mut query: Query<(&mut Velocity, &GroundDetection, &mut JumpBuffer, &PlayerStats)>,
) {
    for (mut velocity, ground, mut buffer, stats) in &mut query {
        if ground.is_grounded && buffer.buffered {
            velocity.y = stats.jump_force;
            buffer.buffered = false;
        }
    }
}
```

#### 3.3 可変ジャンプ高さ
ボタンを押し続ける時間でジャンプの高さが変わる

- [ ] ジャンプボタン押下時間の計測
- [ ] 短押しで低ジャンプ、長押しで高ジャンプ

```rust
#[derive(Component)]
pub struct VariableJump {
    pub min_jump_time: f32,
    pub max_jump_time: f32,
    pub jump_timer: f32,
    pub is_jumping: bool,
}

fn variable_jump_height(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut VariableJump)>,
) {
    for (mut velocity, mut jump) in &mut query {
        if jump.is_jumping {
            jump.jump_timer += time.delta_seconds();

            // ボタンを離したら上昇を弱める
            if !keyboard.pressed(KeyCode::Space) {
                if jump.jump_timer < jump.min_jump_time {
                    velocity.y *= 0.5; // 短押しは低ジャンプ
                }
                jump.is_jumping = false;
            }

            // 最大時間を超えたら自動で終了
            if jump.jump_timer >= jump.max_jump_time {
                jump.is_jumping = false;
            }
        }

        // ジャンプ開始時
        if keyboard.just_pressed(KeyCode::Space) {
            jump.is_jumping = true;
            jump.jump_timer = 0.0;
        }
    }
}
```

#### 3.4 多段ジャンプ（ダブルジャンプ）
- [ ] ジャンプ回数カウンター
- [ ] 空中での追加ジャンプ
- [ ] リセット処理

```rust
#[derive(Component)]
pub struct MultiJump {
    pub max_jumps: u8,
    pub current_jumps: u8,
}

fn multi_jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut MultiJump, &GroundDetection, &PlayerStats)>,
) {
    for (mut velocity, mut multi_jump, ground, stats) in &mut query {
        // 着地時にリセット
        if ground.is_grounded {
            multi_jump.current_jumps = 0;
        }

        // ジャンプ入力
        if keyboard.just_pressed(KeyCode::Space) {
            if multi_jump.current_jumps < multi_jump.max_jumps {
                velocity.y = stats.jump_force;
                multi_jump.current_jumps += 1;
            }
        }
    }
}
```

#### 3.5 ダッシュ/ロール
- [ ] ダッシュ入力の処理
- [ ] ダッシュ速度の適用
- [ ] ダッシュクールダウン
- [ ] ダッシュ中の無敵時間（オプション）

```rust
#[derive(Component)]
pub struct Dash {
    pub speed: f32,
    pub duration: f32,
    pub cooldown: f32,
    pub timer: Timer,
    pub cooldown_timer: Timer,
    pub is_dashing: bool,
}

fn player_dash(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Dash, &Player)>,
) {
    for (mut velocity, mut dash, player) in &mut query {
        dash.timer.tick(time.delta());
        dash.cooldown_timer.tick(time.delta());

        // ダッシュ中
        if dash.is_dashing {
            if dash.timer.finished() {
                dash.is_dashing = false;
            }
        }
        // ダッシュ開始
        else if keyboard.just_pressed(KeyCode::ShiftLeft) && dash.cooldown_timer.finished() {
            dash.is_dashing = true;
            dash.timer.reset();
            dash.cooldown_timer.reset();

            let direction = if player.facing_right { 1.0 } else { -1.0 };
            velocity.x = dash.speed * direction;
        }
    }
}
```

#### 3.6 壁ジャンプ
- [ ] 壁との接触判定
- [ ] 壁張り付き状態
- [ ] 壁からのジャンプ

```rust
#[derive(Component)]
pub struct WallSlide {
    pub is_on_wall: bool,
    pub wall_on_right: bool,
    pub slide_speed: f32,
}

fn wall_detection(
    mut query: Query<(&Transform, &Collider, &mut WallSlide), With<Player>>,
    wall_query: Query<(&Transform, &Collider), With<Wall>>,
) {
    for (player_transform, player_collider, mut wall_slide) in &mut query {
        wall_slide.is_on_wall = false;

        // 壁との衝突チェック（左右のみ）
        for (wall_transform, wall_collider) in &wall_query {
            if check_wall_collision(player_transform, player_collider, wall_transform, wall_collider) {
                wall_slide.is_on_wall = true;
                wall_slide.wall_on_right = wall_transform.translation.x > player_transform.translation.x;
            }
        }
    }
}

fn wall_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &WallSlide, &PlayerStats)>,
) {
    for (mut velocity, wall_slide, stats) in &mut query {
        if wall_slide.is_on_wall && keyboard.just_pressed(KeyCode::Space) {
            // 壁と反対方向にジャンプ
            let direction = if wall_slide.wall_on_right { -1.0 } else { 1.0 };
            velocity.x = stats.move_speed * direction * 1.2;
            velocity.y = stats.jump_force;
        }
    }
}
```

#### 3.7 入力バッファリングシステム（汎用）
- [ ] 入力履歴の記録
- [ ] バッファ時間管理
- [ ] 複数入力の対応

```rust
#[derive(Resource)]
pub struct InputBuffer {
    pub buffer_time: f32,
    pub inputs: VecDeque<BufferedInput>,
}

pub struct BufferedInput {
    pub action: PlayerAction,
    pub timestamp: f32,
}

pub enum PlayerAction {
    Jump,
    Attack,
    Dash,
    Ability,
}
```

### 成果物
- 気持ちいい移動感覚
- コヨーテタイム/ジャンプバッファで許容度の高い操作
- ダブルジャンプ、ダッシュ、壁ジャンプが使える

### 学習リソース
- [Celeste Movement Analysis](https://www.youtube.com/watch?v=yorTG9at90g)
- [Game Feel: A Game Designer's Guide to Virtual Sensation](https://www.amazon.com/dp/0123743281)

---

## Phase 4: アニメーションシステム（予想時間: 3-4日）

### 目標
スプライトアニメーションを実装し、動きに生命感を与える

### タスク

#### 4.1 テクスチャアトラスのセットアップ
- [ ] スプライトシートの読み込み
- [ ] TextureAtlasの作成
- [ ] アセット管理システム

```rust
#[derive(Resource)]
pub struct CharacterAssets {
    pub fox: Handle<TextureAtlasLayout>,
    pub fox_texture: Handle<Image>,
}

fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let fox_texture = asset_server.load("graphics/characters/players/fox/spritesheets/fox.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),  // タイルサイズ
        6,  // 列数
        10, // 行数
        None,
        None,
    );
    let fox_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(CharacterAssets {
        fox: fox_layout,
        fox_texture,
    });
}
```

#### 4.2 アニメーションコンポーネント
- [ ] AnimationControllerコンポーネント
- [ ] AnimationStateコンポーネント
- [ ] AnimationClip定義

```rust
#[derive(Component)]
pub struct AnimationController {
    pub animations: HashMap<String, AnimationClip>,
    pub current_animation: String,
    pub previous_animation: String,
}

#[derive(Component)]
pub struct AnimationState {
    pub current_frame: usize,
    pub timer: Timer,
    pub playing: bool,
    pub looping: bool,
}

#[derive(Clone)]
pub struct AnimationClip {
    pub first_frame: usize,
    pub last_frame: usize,
    pub fps: f32,
}

impl AnimationController {
    pub fn play(&mut self, animation: &str, state: &mut AnimationState) {
        if let Some(clip) = self.animations.get(animation) {
            if self.current_animation != animation {
                self.previous_animation = self.current_animation.clone();
                self.current_animation = animation.to_string();
                state.current_frame = clip.first_frame;
                state.timer.reset();
            }
        }
    }
}
```

#### 4.3 アニメーション更新システム
- [ ] フレーム更新ロジック
- [ ] ループ処理
- [ ] TextureAtlasの更新

```rust
fn update_animations(
    time: Res<Time>,
    mut query: Query<(
        &AnimationController,
        &mut AnimationState,
        &mut Sprite,
    )>,
) {
    for (controller, mut state, mut sprite) in &mut query {
        if !state.playing {
            continue;
        }

        state.timer.tick(time.delta());

        if state.timer.just_finished() {
            let clip = controller.animations.get(&controller.current_animation).unwrap();

            state.current_frame += 1;

            if state.current_frame > clip.last_frame {
                if state.looping {
                    state.current_frame = clip.first_frame;
                } else {
                    state.current_frame = clip.last_frame;
                    state.playing = false;
                }
            }

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = state.current_frame;
            }
        }
    }
}
```

#### 4.4 状態に応じたアニメーション遷移
- [ ] Idle状態
- [ ] Walk/Run状態
- [ ] Jump状態（上昇/下降）
- [ ] Fall状態
- [ ] Land状態

```rust
fn player_animation_controller(
    mut query: Query<(
        &Velocity,
        &GroundDetection,
        &mut AnimationController,
        &mut AnimationState,
    ), With<Player>>,
) {
    for (velocity, ground, mut controller, mut state) in &mut query {
        let animation = if ground.is_grounded {
            if velocity.x.abs() > 10.0 {
                "run"
            } else {
                "idle"
            }
        } else {
            if velocity.y > 50.0 {
                "jump"
            } else {
                "fall"
            }
        };

        controller.play(animation, &mut state);
    }
}
```

#### 4.5 アニメーションイベント
- [ ] 特定フレームでのイベント発火
- [ ] 足音、エフェクトのトリガー

```rust
#[derive(Component)]
pub struct AnimationEvents {
    pub events: HashMap<usize, Vec<AnimationEvent>>,
}

pub enum AnimationEvent {
    PlaySound(String),
    SpawnEffect(String),
    TriggerCallback(fn()),
}

fn process_animation_events(
    mut query: Query<(&AnimationState, &AnimationEvents)>,
    mut commands: Commands,
) {
    for (state, events) in &mut query {
        if let Some(frame_events) = events.events.get(&state.current_frame) {
            for event in frame_events {
                match event {
                    AnimationEvent::PlaySound(sound) => {
                        // 音を鳴らす
                    }
                    AnimationEvent::SpawnEffect(effect) => {
                        // エフェクトを生成
                    }
                    AnimationEvent::TriggerCallback(callback) => {
                        callback();
                    }
                }
            }
        }
    }
}
```

#### 4.6 RONからのアニメーション読み込み
- [ ] RON設定の構造体
- [ ] パース処理
- [ ] AnimationControllerへの変換

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnimationConfigRon {
    pub spritesheet_path: String,
    pub sprite_size: (f32, f32),
    pub columns: usize,
    pub rows: usize,
    pub clips: HashMap<String, AnimationClipRon>,
    pub default_animation: String,
}

#[derive(Deserialize)]
pub struct AnimationClipRon {
    pub first: usize,
    pub last: usize,
    pub fps: f32,
}

impl From<AnimationConfigRon> for AnimationController {
    fn from(config: AnimationConfigRon) -> Self {
        let mut animations = HashMap::new();

        for (name, clip_ron) in config.clips {
            animations.insert(name, AnimationClip {
                first_frame: clip_ron.first,
                last_frame: clip_ron.last,
                fps: clip_ron.fps,
            });
        }

        AnimationController {
            animations,
            current_animation: config.default_animation.clone(),
            previous_animation: String::new(),
        }
    }
}
```

### 成果物
- プレイヤーの動きに応じたアニメーション
- RONファイルでアニメーション設定が管理できる

---

## Phase 5: 戦闘基礎（予想時間: 3-4日）

### 目標
基本的な攻撃とダメージシステムを実装

### タスク

#### 5.1 体力システム
- [ ] Healthコンポーネント
- [ ] ダメージ処理
- [ ] 死亡判定

```rust
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
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
}

fn apply_damage(
    mut events: EventReader<DamageEvent>,
    mut query: Query<&mut Health>,
    mut death_events: EventWriter<DeathEvent>,
) {
    for event in events.read() {
        if let Ok(mut health) = query.get_mut(event.target) {
            health.current -= event.amount;

            if health.current <= 0.0 {
                death_events.send(DeathEvent {
                    entity: event.target,
                });
            }
        }
    }
}
```

#### 5.2 基本的な攻撃
- [ ] 攻撃入力の処理
- [ ] 攻撃判定の生成
- [ ] ヒットボックス/ハートボックス

```rust
#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub active: bool,
    pub duration: Timer,
    pub hitbox: Collider,
}

#[derive(Component)]
pub struct Hitbox {
    pub damage: f32,
    pub knockback: Vec2,
    pub hit_entities: HashSet<Entity>,  // 同じ敵を複数回ヒットさせない
}

fn player_attack(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Player), With<Player>>,
) {
    for (entity, transform, player) in &query {
        if keyboard.just_pressed(KeyCode::KeyX) {
            // 攻撃判定エンティティを生成
            let offset = if player.facing_right { 20.0 } else { -20.0 };

            commands.spawn((
                Hitbox {
                    damage: 10.0,
                    knockback: Vec2::new(offset * 5.0, 100.0),
                    hit_entities: HashSet::new(),
                },
                Collider {
                    size: Vec2::new(30.0, 30.0),
                    offset: Vec2::new(offset, 0.0),
                },
                Transform::from_translation(transform.translation),
                AttackLifetime(Timer::from_seconds(0.2, TimerMode::Once)),
            ));
        }
    }
}

#[derive(Component)]
pub struct AttackLifetime(Timer);

fn despawn_expired_attacks(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AttackLifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0.tick(time.delta());
        if lifetime.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

#### 5.3 ヒット検出
- [ ] 攻撃判定と敵の衝突検出
- [ ] ダメージイベントの発火
- [ ] 重複ヒットの防止

```rust
fn detect_hits(
    mut hitbox_query: Query<(&Transform, &Collider, &mut Hitbox)>,
    target_query: Query<(Entity, &Transform, &Collider), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (hitbox_transform, hitbox_collider, mut hitbox) in &mut hitbox_query {
        for (enemy_entity, enemy_transform, enemy_collider) in &target_query {
            // 既にヒットしている場合はスキップ
            if hitbox.hit_entities.contains(&enemy_entity) {
                continue;
            }

            if check_collision(hitbox_transform, hitbox_collider, enemy_transform, enemy_collider) {
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    amount: hitbox.damage,
                    source: DamageSource::Player,
                    knockback: hitbox.knockback,
                });

                hitbox.hit_entities.insert(enemy_entity);
            }
        }
    }
}
```

#### 5.4 ノックバック
- [ ] ダメージ時の吹き飛び
- [ ] ノックバック方向の計算
- [ ] ノックバック中の制御不能

```rust
#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub duration: Timer,
    pub active: bool,
}

fn apply_knockback_from_damage(
    mut events: EventReader<DamageEvent>,
    mut query: Query<&mut Knockback>,
) {
    for event in events.read() {
        if let Ok(mut knockback) = query.get_mut(event.target) {
            knockback.velocity = event.knockback;
            knockback.active = true;
            knockback.duration.reset();
        }
    }
}

fn update_knockback(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Knockback)>,
) {
    for (mut velocity, mut knockback) in &mut query {
        if knockback.active {
            knockback.duration.tick(time.delta());

            // ノックバック速度を適用
            velocity.x = knockback.velocity.x;
            velocity.y = knockback.velocity.y;

            // 減衰
            knockback.velocity *= 0.9;

            if knockback.duration.finished() {
                knockback.active = false;
            }
        }
    }
}
```

#### 5.5 無敵時間
- [ ] ダメージ後の一時的無敵
- [ ] 無敵中の点滅エフェクト
- [ ] ダメージ無効化

```rust
#[derive(Component)]
pub struct Invincibility {
    pub timer: Timer,
    pub active: bool,
    pub blink_interval: f32,
    pub blink_timer: f32,
}

fn start_invincibility_on_damage(
    mut events: EventReader<DamageEvent>,
    mut query: Query<&mut Invincibility>,
) {
    for event in events.read() {
        if let Ok(mut invincibility) = query.get_mut(event.target) {
            invincibility.active = true;
            invincibility.timer.reset();
        }
    }
}

fn update_invincibility(
    time: Res<Time>,
    mut query: Query<(&mut Invincibility, &mut Visibility)>,
) {
    for (mut invincibility, mut visibility) in &mut query {
        if invincibility.active {
            invincibility.timer.tick(time.delta());
            invincibility.blink_timer += time.delta_seconds();

            // 点滅
            if invincibility.blink_timer >= invincibility.blink_interval {
                *visibility = match *visibility {
                    Visibility::Visible => Visibility::Hidden,
                    _ => Visibility::Visible,
                };
                invincibility.blink_timer = 0.0;
            }

            if invincibility.timer.finished() {
                invincibility.active = false;
                *visibility = Visibility::Visible;
            }
        }
    }
}

fn ignore_damage_when_invincible(
    mut events: EventReader<DamageEvent>,
    query: Query<&Invincibility>,
    mut filtered_events: EventWriter<DamageEvent>,
) {
    for event in events.read() {
        if let Ok(invincibility) = query.get(event.target) {
            if !invincibility.active {
                filtered_events.send(event.clone());
            }
        }
    }
}
```

#### 5.6 死亡処理
- [ ] 死亡アニメーション
- [ ] リスポーン
- [ ] ライフシステム

```rust
#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Lives {
    pub count: u8,
}

fn handle_player_death(
    mut events: EventReader<DeathEvent>,
    mut query: Query<(&mut Lives, &mut Transform, &mut Health), With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        if let Ok((mut lives, mut transform, mut health)) = query.get_mut(event.entity) {
            lives.count = lives.count.saturating_sub(1);

            if lives.count > 0 {
                // リスポーン
                transform.translation = Vec3::new(0.0, 0.0, 0.0); // チェックポイント位置
                health.current = health.max;
            } else {
                // ゲームオーバー
                game_state.set(GameState::GameOver);
            }
        }
    }
}
```

### 成果物
- 攻撃ボタンで敵を攻撃できる
- ダメージを受けるとノックバックと無敵時間が発生
- 体力がゼロになると死亡処理

---

## Phase 6: 敵システムI（予想時間: 4-5日）

### 目標
基本的な敵AIとスポーンシステムを実装

### タスク

#### 6.1 敵エンティティのスポーン
- [ ] Enemyコンポーネント
- [ ] RON設定からの読み込み
- [ ] スポーンシステム

```rust
#[derive(Component)]
pub struct Enemy {
    pub enemy_type: String,
}

#[derive(Component)]
pub struct EnemyStats {
    pub move_speed: f32,
    pub damage: f32,
    pub score_value: u32,
}

fn spawn_enemy(
    commands: &mut Commands,
    enemy_type: &str,
    position: Vec2,
    configs: &GameConfigs,
    assets: &GameAssets,
) {
    let config = configs.enemies.get(enemy_type).unwrap();

    commands.spawn((
        Enemy {
            enemy_type: enemy_type.to_string(),
        },
        EnemyStats {
            move_speed: config.stats.move_speed,
            damage: config.stats.damage,
            score_value: config.stats.score_value,
        },
        Health {
            current: config.health.max,
            max: config.health.max,
        },
        Velocity { x: 0.0, y: 0.0 },
        Collider {
            size: config.collider.size,
            offset: config.collider.offset,
        },
        Transform::from_translation(position.extend(0.0)),
        // ... アニメーションコンポーネント等
    ));
}
```

#### 6.2 基本的なAI - Patrol（巡回）
- [ ] 巡回ポイントの設定
- [ ] 移動ロジック
- [ ] 方向転換

```rust
#[derive(Component)]
pub struct PatrolAI {
    pub patrol_points: Vec<Vec2>,
    pub current_point_index: usize,
    pub wait_time: f32,
    pub wait_timer: f32,
}

fn patrol_ai_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut PatrolAI, &EnemyStats)>,
) {
    for (mut transform, mut velocity, mut patrol, stats) in &mut query {
        let current_pos = transform.translation.truncate();
        let target_pos = patrol.patrol_points[patrol.current_point_index];
        let distance = current_pos.distance(target_pos);

        if distance < 5.0 {
            // 目標地点に到達
            if patrol.wait_timer >= patrol.wait_time {
                // 次のポイントへ
                patrol.current_point_index = (patrol.current_point_index + 1) % patrol.patrol_points.len();
                patrol.wait_timer = 0.0;
            } else {
                patrol.wait_timer += time.delta_seconds();
                velocity.x = 0.0;
            }
        } else {
            // 移動
            let direction = (target_pos - current_pos).normalize();
            velocity.x = direction.x * stats.move_speed;
        }
    }
}
```

#### 6.3 Chase AI（追跡）
- [ ] プレイヤー検知
- [ ] 追跡ロジック
- [ ] 視界範囲

```rust
#[derive(Component)]
pub struct ChaseAI {
    pub detection_range: f32,
    pub attack_range: f32,
    pub is_chasing: bool,
}

fn chase_ai_system(
    mut enemy_query: Query<(&Transform, &mut Velocity, &mut ChaseAI, &EnemyStats), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation.truncate();

        for (enemy_transform, mut velocity, mut chase, stats) in &mut enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = enemy_pos.distance(player_pos);

            if distance < chase.detection_range {
                chase.is_chasing = true;

                if distance > chase.attack_range {
                    // プレイヤーに向かって移動
                    let direction = (player_pos - enemy_pos).normalize();
                    velocity.x = direction.x * stats.move_speed;
                } else {
                    // 攻撃範囲内では停止
                    velocity.x = 0.0;
                }
            } else {
                chase.is_chasing = false;
                velocity.x = 0.0;
            }
        }
    }
}
```

#### 6.4 Flying AI（飛行パターン）
- [ ] サインカーブ移動
- [ ] 円運動
- [ ] ホバリング

```rust
#[derive(Component)]
pub struct FlyingAI {
    pub pattern: FlyingPattern,
    pub time: f32,
    pub origin: Vec2,
}

pub enum FlyingPattern {
    SineWave { amplitude: f32, frequency: f32 },
    Circle { radius: f32, speed: f32 },
    Hover { range: f32, speed: f32 },
}

fn flying_ai_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut FlyingAI, &EnemyStats)>,
) {
    for (mut transform, mut flying, stats) in &mut query {
        flying.time += time.delta_seconds();

        match &flying.pattern {
            FlyingPattern::SineWave { amplitude, frequency } => {
                let x = flying.origin.x + flying.time * stats.move_speed;
                let y = flying.origin.y + (flying.time * frequency).sin() * amplitude;
                transform.translation = Vec3::new(x, y, 0.0);
            }
            FlyingPattern::Circle { radius, speed } => {
                let angle = flying.time * speed;
                let x = flying.origin.x + angle.cos() * radius;
                let y = flying.origin.y + angle.sin() * radius;
                transform.translation = Vec3::new(x, y, 0.0);
            }
            FlyingPattern::Hover { range, speed } => {
                let offset = (flying.time * speed).sin() * range;
                transform.translation.y = flying.origin.y + offset;
            }
        }
    }
}
```

#### 6.5 敵の攻撃
- [ ] プレイヤーへの接触ダメージ
- [ ] 弾発射（飛び道具）

```rust
#[derive(Component)]
pub struct ContactDamage {
    pub damage: f32,
    pub knockback_force: f32,
}

fn enemy_contact_damage(
    enemy_query: Query<(&Transform, &Collider, &ContactDamage), With<Enemy>>,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (player_entity, player_transform, player_collider) in &player_query {
        for (enemy_transform, enemy_collider, contact) in &enemy_query {
            if check_collision(player_transform, player_collider, enemy_transform, enemy_collider) {
                let direction = (player_transform.translation - enemy_transform.translation).normalize();
                let knockback = direction.truncate() * contact.knockback_force;

                damage_events.send(DamageEvent {
                    target: player_entity,
                    amount: contact.damage,
                    source: DamageSource::Enemy(player_entity),
                    knockback,
                });
            }
        }
    }
}
```

#### 6.6 敵の死亡処理
- [ ] 死亡アニメーション
- [ ] スコア加算
- [ ] ドロップアイテム

```rust
fn handle_enemy_death(
    mut commands: Commands,
    mut events: EventReader<DeathEvent>,
    query: Query<(&Transform, &EnemyStats), With<Enemy>>,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        if let Ok((transform, stats)) = query.get(event.entity) {
            // スコア加算
            score.value += stats.score_value;

            // エフェクト生成
            spawn_death_effect(&mut commands, transform.translation.truncate());

            // ドロップアイテム（確率）
            if rand::random::<f32>() < 0.3 {
                spawn_collectible(&mut commands, transform.translation.truncate());
            }

            // エンティティ削除
            commands.entity(event.entity).despawn_recursive();
        }
    }
}
```

### 成果物
- 3種類のAI（巡回、追跡、飛行）を持つ敵
- 敵がプレイヤーを攻撃
- 敵を倒すとスコアが入る

---

## Phase 7: レベルシステム基礎（予想時間: 5-6日）

### 目標
タイルマップベースのレベルシステムを構築

### タスク

#### 7.1 bevy_ecs_tilemapの導入
- [ ] Cargo.tomlに追加
- [ ] 基本的なタイルマップの表示

```toml
[dependencies]
bevy_ecs_tilemap = "0.15"
```

```rust
use bevy_ecs_tilemap::prelude::*;

fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("graphics/environments/sunny_land/tilesets/tileset.png");

    let map_size = TilemapSize { x: 32, y: 18 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // タイルを配置
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0), // タイル番号
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });
}
```

#### 7.2 レベルデータのRON化
- [ ] タイルマップデータの設計
- [ ] RONファイルからの読み込み
- [ ] タイルの種類（地面、壁、空）

```ron
// assets/config/levels/stage_1_1.ron
(
    name: "Forest Path",
    width: 32,
    height: 18,

    // タイルマップデータ（2D配列）
    tiles: [
        [0, 0, 0, 0, 0, 0, 0, 0, ...],  // 一番上の行
        [0, 0, 0, 0, 0, 0, 0, 0, ...],
        // ... 18行
    ],

    // タイルの衝突情報
    collision_tiles: [1, 2, 3, 5, 6],  // これらのタイル番号は衝突判定あり

    // エンティティ配置
    entities: [
        (type: "enemy_bat", position: (x: 300.0, y: 150.0)),
        (type: "collectible_cherry", position: (x: 200.0, y: 180.0)),
    ],
)
```

#### 7.3 タイルとの衝突判定
- [ ] タイルベースの衝突判定
- [ ] 地面判定の改良
- [ ] 壁との衝突

```rust
#[derive(Component)]
pub struct TileCollider;

fn tilemap_collision(
    mut player_query: Query<(&mut Transform, &mut Velocity, &Collider), With<Player>>,
    tilemap_query: Query<(&TileStorage, &TilemapSize, &TilemapGridSize, &TilemapType)>,
    tile_query: Query<&TilePos, With<TileCollider>>,
) {
    for (mut transform, mut velocity, collider) in &mut player_query {
        for (tile_storage, map_size, grid_size, map_type) in &tilemap_query {
            // プレイヤーが重なっているタイルを取得
            let player_pos = transform.translation.truncate();

            // 周囲のタイルをチェック
            let check_tiles = get_surrounding_tiles(player_pos, grid_size);

            for tile_pos in check_tiles {
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    if tile_query.contains(tile_entity) {
                        // 衝突処理
                        resolve_tile_collision(&mut transform, &mut velocity, collider, &tile_pos, grid_size);
                    }
                }
            }
        }
    }
}
```

#### 7.4 背景レイヤー（視差効果）
- [ ] 複数の背景レイヤー
- [ ] 視差スクロール
- [ ] 深度管理

```rust
#[derive(Component)]
pub struct ParallaxLayer {
    pub speed: f32,
    pub depth: f32,
}

fn parallax_background(
    camera_query: Query<&Transform, With<Camera>>,
    mut background_query: Query<(&mut Transform, &ParallaxLayer), Without<Camera>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_pos = camera_transform.translation.truncate();

        for (mut bg_transform, parallax) in &mut background_query {
            // カメラ移動量に視差速度を掛ける
            bg_transform.translation.x = -camera_pos.x * parallax.speed;
            bg_transform.translation.y = -camera_pos.y * parallax.speed * 0.5; // Y方向は半分
            bg_transform.translation.z = parallax.depth;
        }
    }
}
```

#### 7.5 ゴール/クリア判定
- [ ] ゴール地点の配置
- [ ] ゴール到達の検知
- [ ] ステージクリア処理

```rust
#[derive(Component)]
pub struct Goal;

fn check_goal_reached(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    goal_query: Query<(&Transform, &Collider), With<Goal>>,
    mut stage_events: EventWriter<StageCompleteEvent>,
) {
    for (player_transform, player_collider) in &player_query {
        for (goal_transform, goal_collider) in &goal_query {
            if check_collision(player_transform, player_collider, goal_transform, goal_collider) {
                stage_events.send(StageCompleteEvent);
            }
        }
    }
}
```

#### 7.6 チェックポイント
- [ ] チェックポイント配置
- [ ] 通過時のセーブ
- [ ] リスポーン位置の更新

```rust
#[derive(Component)]
pub struct Checkpoint {
    pub activated: bool,
}

#[derive(Resource)]
pub struct RespawnPoint(pub Vec2);

fn checkpoint_system(
    mut player_query: Query<&Transform, With<Player>>,
    mut checkpoint_query: Query<(&Transform, &mut Checkpoint)>,
    mut respawn_point: ResMut<RespawnPoint>,
) {
    let player_pos = player_query.single().translation.truncate();

    for (checkpoint_transform, mut checkpoint) in &mut checkpoint_query {
        if checkpoint.activated {
            continue;
        }

        let checkpoint_pos = checkpoint_transform.translation.truncate();
        if player_pos.distance(checkpoint_pos) < 30.0 {
            checkpoint.activated = true;
            respawn_point.0 = checkpoint_pos;
            // エフェクト/サウンド再生
        }
    }
}
```

### 成果物
- タイルマップで構築されたステージ
- 視差効果のある背景
- ゴールとチェックポイント

### 学習リソース
- [bevy_ecs_tilemap ドキュメント](https://github.com/StarArawn/bevy_ecs_tilemap)

---

## Phase 8: カメラシステムI（予想時間: 2-3日）

### 目標
気持ちいいカメラワークを実装

### タスク

#### 8.1 スムーズなプレイヤー追従
- [ ] Lerpによる滑らかな追従
- [ ] デッドゾーン（中央付近では動かない）

```rust
#[derive(Component)]
pub struct CameraFollow {
    pub target: Entity,
    pub smoothness: f32,
    pub dead_zone: Rect,
}

fn camera_follow_system(
    time: Res<Time>,
    target_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &CameraFollow), Without<Player>>,
) {
    for (mut camera_transform, follow) in &mut camera_query {
        if let Ok(target_transform) = target_query.get(follow.target) {
            let target_pos = target_transform.translation.truncate();
            let camera_pos = camera_transform.translation.truncate();

            // デッドゾーン外なら追従
            let offset = target_pos - camera_pos;

            let should_move_x = offset.x.abs() > follow.dead_zone.width() / 2.0;
            let should_move_y = offset.y.abs() > follow.dead_zone.height() / 2.0;

            let mut new_pos = camera_pos;

            if should_move_x {
                new_pos.x = new_pos.x.lerp(target_pos.x, follow.smoothness * time.delta_seconds());
            }
            if should_move_y {
                new_pos.y = new_pos.y.lerp(target_pos.y, follow.smoothness * time.delta_seconds());
            }

            camera_transform.translation = new_pos.extend(camera_transform.translation.z);
        }
    }
}
```

#### 8.2 カメラ境界
- [ ] ステージの端でカメラを止める
- [ ] 最小/最大座標の設定

```rust
#[derive(Component)]
pub struct CameraBounds {
    pub min: Vec2,
    pub max: Vec2,
}

fn apply_camera_bounds(
    mut camera_query: Query<(&mut Transform, &CameraBounds), With<Camera>>,
) {
    for (mut transform, bounds) in &mut camera_query {
        transform.translation.x = transform.translation.x.clamp(bounds.min.x, bounds.max.x);
        transform.translation.y = transform.translation.y.clamp(bounds.min.y, bounds.max.y);
    }
}
```

#### 8.3 カメラシェイク
- [ ] ダメージ時のシェイク
- [ ] ランダムなオフセット
- [ ] 減衰

```rust
#[derive(Component)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: Timer,
    pub active: bool,
}

fn camera_shake_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CameraShake), With<Camera>>,
) {
    for (mut transform, mut shake) in &mut query {
        if shake.active {
            shake.duration.tick(time.delta());

            if shake.duration.finished() {
                shake.active = false;
            } else {
                // ランダムなオフセット
                let progress = shake.duration.fraction();
                let current_intensity = shake.intensity * (1.0 - progress); // 減衰

                let offset_x = (rand::random::<f32>() - 0.5) * 2.0 * current_intensity;
                let offset_y = (rand::random::<f32>() - 0.5) * 2.0 * current_intensity;

                transform.translation.x += offset_x;
                transform.translation.y += offset_y;
            }
        }
    }
}

// ダメージイベントでシェイクを開始
fn trigger_shake_on_damage(
    mut events: EventReader<DamageEvent>,
    mut camera_query: Query<&mut CameraShake, With<Camera>>,
) {
    for _event in events.read() {
        if let Ok(mut shake) = camera_query.get_single_mut() {
            shake.active = true;
            shake.intensity = 10.0;
            shake.duration.reset();
        }
    }
}
```

#### 8.4 ピクセルパーフェクト設定
- [ ] 内部解像度の設定
- [ ] 拡大時のピクセルアート維持

```rust
fn setup_pixel_perfect_camera(
    mut commands: Commands,
) {
    // 内部解像度320x180を1280x720に拡大
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(4.0), // 4倍スケール
                ..default()
            },
            ..default()
        },
        CameraFollow {
            target: Entity::PLACEHOLDER, // 後でプレイヤーエンティティを設定
            smoothness: 5.0,
            dead_zone: Rect::from_center_size(Vec2::ZERO, Vec2::new(100.0, 60.0)),
        },
        CameraBounds {
            min: Vec2::new(-500.0, -300.0),
            max: Vec2::new(500.0, 300.0),
        },
        CameraShake {
            intensity: 0.0,
            duration: Timer::from_seconds(0.3, TimerMode::Once),
            active: false,
        },
    ));
}
```

### 成果物
- 滑らかにプレイヤーを追従するカメラ
- ステージ境界でのカメラ停止
- ダメージ時のカメラシェイク

---

## Phase 9: ビジュアルエフェクトI（予想時間: 3-4日）

### 目標
基本的なエフェクトシステムを実装

### タスク

#### 9.1 パーティクルシステムの基礎
- [ ] パーティクルエンティティ
- [ ] エミッター
- [ ] ライフタイムと消滅

```rust
#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub initial_velocity: Vec2,
    pub gravity_scale: f32,
    pub fade_out: bool,
}

#[derive(Bundle)]
pub struct ParticleBundle {
    pub particle: Particle,
    pub velocity: Velocity,
    pub sprite: SpriteBundle,
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Velocity, &mut Sprite)>,
) {
    for (entity, mut particle, mut velocity, mut sprite) in &mut query {
        particle.lifetime.tick(time.delta());

        // 重力適用
        velocity.y -= 200.0 * particle.gravity_scale * time.delta_seconds();

        // フェードアウト
        if particle.fade_out {
            let progress = particle.lifetime.fraction();
            if let Some(color) = &mut sprite.color {
                color.set_alpha(1.0 - progress);
            }
        }

        // 寿命が尽きたら削除
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_particle_burst(
    commands: &mut Commands,
    position: Vec2,
    count: usize,
    color: Color,
) {
    for _ in 0..count {
        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let speed = 50.0 + rand::random::<f32>() * 100.0;

        commands.spawn(ParticleBundle {
            particle: Particle {
                lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                initial_velocity: Vec2::new(angle.cos(), angle.sin()) * speed,
                gravity_scale: 0.5,
                fade_out: true,
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(4.0, 4.0)),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(1.0)),
                ..default()
            },
        });
    }
}
```

#### 9.2 ヒットエフェクト
- [ ] 攻撃ヒット時のエフェクト
- [ ] スラッシュエフェクト
- [ ] インパクトフレーム

```rust
fn spawn_hit_effect(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventReader<DamageEvent>,
    query: Query<&Transform>,
) {
    for event in events.read() {
        if let Ok(transform) = query.get(event.target) {
            // パーティクルバースト
            spawn_particle_burst(
                &mut commands,
                transform.translation.truncate(),
                10,
                Color::srgb(1.0, 0.8, 0.0),
            );

            // スラッシュスプライト
            commands.spawn((
                SpriteBundle {
                    texture: assets.slash_effect.clone(),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
                EffectLifetime(Timer::from_seconds(0.3, TimerMode::Once)),
                AnimationState {
                    current_frame: 0,
                    timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                    playing: true,
                    looping: false,
                },
            ));
        }
    }
}
```

#### 9.3 ヒットストップ
画面を一瞬止めてインパクトを強調

- [ ] 時間の停止
- [ ] 対象エンティティの選別
- [ ] 短時間での再開

```rust
#[derive(Resource)]
pub struct HitStop {
    pub duration: Timer,
    pub active: bool,
}

fn trigger_hitstop(
    mut events: EventReader<DamageEvent>,
    mut hitstop: ResMut<HitStop>,
) {
    for _event in events.read() {
        hitstop.active = true;
        hitstop.duration = Timer::from_seconds(0.1, TimerMode::Once);
    }
}

fn update_hitstop(
    real_time: Res<Time<Real>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut hitstop: ResMut<HitStop>,
) {
    if hitstop.active {
        hitstop.duration.tick(real_time.delta());

        if hitstop.duration.finished() {
            hitstop.active = false;
            virtual_time.unpause();
        } else {
            virtual_time.pause();
        }
    }
}
```

#### 9.4 トレイルエフェクト
- [ ] 残像の生成
- [ ] フェードアウト
- [ ] ダッシュ時のトレイル

```rust
#[derive(Component)]
pub struct Trail {
    pub spawn_timer: Timer,
    pub trail_lifetime: f32,
}

fn spawn_trail_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &Sprite, &mut Trail)>,
) {
    for (transform, sprite, mut trail) in &mut query {
        trail.spawn_timer.tick(time.delta());

        if trail.spawn_timer.just_finished() {
            // 残像を生成
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(sprite.color.red(), sprite.color.green(), sprite.color.blue(), 0.5),
                        ..sprite.clone()
                    },
                    transform: *transform,
                    ..default()
                },
                TrailGhost {
                    lifetime: Timer::from_seconds(trail.trail_lifetime, TimerMode::Once),
                },
            ));
        }
    }
}

#[derive(Component)]
pub struct TrailGhost {
    pub lifetime: Timer,
}

fn update_trail_ghosts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailGhost, &mut Sprite)>,
) {
    for (entity, mut ghost, mut sprite) in &mut query {
        ghost.lifetime.tick(time.delta());

        // フェードアウト
        let alpha = 1.0 - ghost.lifetime.fraction();
        sprite.color.set_alpha(alpha * 0.5);

        if ghost.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

#### 9.5 ジャンプ/着地エフェクト
- [ ] ジャンプ時の煙
- [ ] 着地時の衝撃波

```rust
fn jump_effect(
    mut commands: Commands,
    query: Query<(&Transform, &GroundDetection), (With<Player>, Changed<GroundDetection>)>,
) {
    for (transform, ground) in &query {
        if !ground.is_grounded {
            // ジャンプ開始
            spawn_particle_burst(
                &mut commands,
                transform.translation.truncate() - Vec2::new(0.0, 16.0),
                5,
                Color::srgba(0.8, 0.8, 0.8, 0.7),
            );
        }
    }
}

fn land_effect(
    mut commands: Commands,
    assets: Res<GameAssets>,
    query: Query<(&Transform, &GroundDetection), (With<Player>, Changed<GroundDetection>)>,
) {
    for (transform, ground) in &query {
        if ground.is_grounded {
            // 着地
            commands.spawn((
                SpriteBundle {
                    texture: assets.land_effect.clone(),
                    transform: Transform::from_translation(
                        transform.translation - Vec3::new(0.0, 16.0, -1.0)
                    ),
                    ..default()
                },
                EffectLifetime(Timer::from_seconds(0.4, TimerMode::Once)),
            ));
        }
    }
}
```

### 成果物
- 攻撃ヒット時の派手なエフェクト
- ヒットストップで重量感のある攻撃
- ダッシュ時のトレイルエフェクト

---

## Phase 10: UIシステムI（予想時間: 3-4日）

### 目標
基本的なゲームUIを実装

### タスク

#### 10.1 HUDレイアウト
- [ ] UI要素の配置
- [ ] Flexbox的なレイアウト

```rust
fn setup_hud(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // 上部UI（体力、スコア）
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 体力表示
                    spawn_health_bar(parent);

                    // スコア表示
                    spawn_score_display(parent);
                });
        });
}
```

#### 10.2 体力バー
- [ ] ハート/バー形式の体力表示
- [ ] スムーズな減少アニメーション

```rust
#[derive(Component)]
pub struct HealthBar {
    pub target_player: Entity,
    pub displayed_health: f32,
    pub animation_speed: f32,
}

fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<(&mut Style, &mut BackgroundColor, &mut HealthBar)>,
) {
    for (mut style, mut bg_color, mut health_bar) in &mut health_bar_query {
        if let Ok(health) = player_query.get(health_bar.target_player) {
            // スムーズに減少
            health_bar.displayed_health = health_bar.displayed_health.lerp(health.current, 0.1);

            let health_percent = (health_bar.displayed_health / health.max).clamp(0.0, 1.0);
            style.width = Val::Percent(health_percent * 100.0);

            // 体力が少ないと赤く
            if health_percent < 0.3 {
                *bg_color = Color::srgb(1.0, 0.0, 0.0).into();
            } else {
                *bg_color = Color::srgb(0.0, 1.0, 0.0).into();
            }
        }
    }
}

fn spawn_health_bar(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgb(0.2, 0.2, 0.2).into(),
            border_color: Color::WHITE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.0, 1.0, 0.0).into(),
                    ..default()
                },
                HealthBar {
                    target_player: Entity::PLACEHOLDER, // 後で設定
                    displayed_health: 100.0,
                    animation_speed: 5.0,
                },
            ));
        });
}
```

#### 10.3 スコア表示
- [ ] リアルタイムスコア更新
- [ ] カウントアップアニメーション

```rust
#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
pub struct ScoreText;

fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Score: {}", score.value);
    }
}

fn spawn_score_display(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/pixelated.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ),
        ScoreText,
    ));
}
```

#### 10.4 ボス体力バー
- [ ] 画面上部のボス専用体力バー
- [ ] 名前表示
- [ ] 登場アニメーション

```rust
#[derive(Component)]
pub struct BossHealthBar {
    pub boss_entity: Entity,
}

fn show_boss_health_bar(
    mut commands: Commands,
    boss_query: Query<(Entity, &Enemy, &Health), Added<Boss>>,
) {
    for (entity, enemy, health) in &boss_query {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(400.0),
                    height: Val::Px(40.0),
                    transform: Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
                ..default()
            })
            .with_children(|parent| {
                // ボス名
                parent.spawn(TextBundle::from_section(
                    &enemy.enemy_type,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));

                // 体力バー
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(10.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.8, 0.0, 0.0).into(),
                        ..default()
                    },
                    BossHealthBar {
                        boss_entity: entity,
                    },
                ));
            });
    }
}
```

#### 10.5 ポーズメニュー
- [ ] ポーズ画面のUI
- [ ] 再開/リトライ/タイトルへ戻る
- [ ] 入力処理

```rust
fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // ボタン
            spawn_menu_button(parent, "Resume", PauseMenuButton::Resume);
            spawn_menu_button(parent, "Retry", PauseMenuButton::Retry);
            spawn_menu_button(parent, "Title", PauseMenuButton::Title);
        });
}

#[derive(Component)]
enum PauseMenuButton {
    Resume,
    Retry,
    Title,
}

fn handle_pause_menu_buttons(
    mut interaction_query: Query<(&Interaction, &PauseMenuButton), Changed<Interaction>>,
    mut game_state: ResMut<NextState<InGameState>>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                PauseMenuButton::Resume => {
                    game_state.set(InGameState::StagePlay);
                }
                PauseMenuButton::Retry => {
                    // ステージをリロード
                }
                PauseMenuButton::Title => {
                    // タイトルへ
                }
            }
        }
    }
}
```

### 成果物
- 体力バー、スコア表示のHUD
- ボス出現時の専用UI
- ポーズメニュー

---

## Phase 11: 能力システム（予想時間: 5-6日）

### 目標
カービィのコピー能力のようなシステムを実装

### タスク

#### 11.1 能力レジストリ
- [ ] 能力定義の構造体
- [ ] RONからの読み込み
- [ ] 能力一覧の管理

```rust
#[derive(Resource)]
pub struct AbilityRegistry {
    pub abilities: HashMap<String, AbilityDefinition>,
}

#[derive(Clone, Deserialize)]
pub struct AbilityDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_path: String,

    pub stats_modifier: StatsModifier,
    pub special_moves: Vec<SpecialMove>,

    pub animation_overrides: HashMap<String, AnimationClipRon>,
}

#[derive(Clone, Deserialize)]
pub struct StatsModifier {
    pub move_speed_multiplier: f32,
    pub jump_force_multiplier: f32,
    pub damage_multiplier: f32,
}

#[derive(Clone, Deserialize)]
pub enum SpecialMove {
    Projectile {
        damage: f32,
        speed: f32,
        sprite: String,
        cooldown: f32,
    },
    Melee {
        damage: f32,
        range: f32,
        knockback: f32,
    },
    Buff {
        duration: f32,
        effect: String,
    },
}
```

#### 11.2 能力取得システム
- [ ] 敵からの能力ドロップ
- [ ] 能力アイテムの配置
- [ ] 取得時の演出

```rust
#[derive(Component)]
pub struct AbilityItem {
    pub ability_id: String,
}

fn pickup_ability(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    ability_item_query: Query<(Entity, &Transform, &Collider, &AbilityItem)>,
    mut ability_events: EventWriter<AbilityGainedEvent>,
) {
    for (player_entity, player_transform, player_collider) in &player_query {
        for (item_entity, item_transform, item_collider, ability_item) in &ability_item_query {
            if check_collision(player_transform, player_collider, item_transform, item_collider) {
                ability_events.send(AbilityGainedEvent {
                    player: player_entity,
                    ability_id: ability_item.ability_id.clone(),
                });

                commands.entity(item_entity).despawn();
            }
        }
    }
}

#[derive(Event)]
pub struct AbilityGainedEvent {
    pub player: Entity,
    pub ability_id: String,
}
```

#### 11.3 能力状態管理
- [ ] 現在の能力の保持
- [ ] 能力の切り替え
- [ ] ステータス補正の適用

```rust
#[derive(Component)]
pub struct CurrentAbility {
    pub ability_id: Option<String>,
}

fn apply_ability_stats(
    mut query: Query<(&CurrentAbility, &mut PlayerStats), Changed<CurrentAbility>>,
    registry: Res<AbilityRegistry>,
) {
    for (current, mut stats) in &mut query {
        // ベースステータスをリセット
        *stats = PlayerStats::default();

        if let Some(ability_id) = &current.ability_id {
            if let Some(ability) = registry.abilities.get(ability_id) {
                stats.move_speed *= ability.stats_modifier.move_speed_multiplier;
                stats.jump_force *= ability.stats_modifier.jump_force_multiplier;
            }
        }
    }
}
```

#### 11.4 特殊技の実装 - 飛び道具

```rust
#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: Timer,
    pub owner: Entity,
}

fn use_projectile_ability(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Player, &CurrentAbility)>,
    registry: Res<AbilityRegistry>,
    assets: Res<GameAssets>,
) {
    for (entity, transform, player, current_ability) in &query {
        if keyboard.just_pressed(KeyCode::KeyC) {
            if let Some(ability_id) = &current_ability.ability_id {
                if let Some(ability) = registry.abilities.get(ability_id) {
                    for special_move in &ability.special_moves {
                        if let SpecialMove::Projectile { damage, speed, sprite, cooldown } = special_move {
                            let direction = if player.facing_right { 1.0 } else { -1.0 };

                            commands.spawn((
                                Projectile {
                                    damage: *damage,
                                    speed: *speed,
                                    lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                                    owner: entity,
                                },
                                Velocity {
                                    x: *speed * direction,
                                    y: 0.0,
                                },
                                SpriteBundle {
                                    texture: assets.projectiles.get(sprite).unwrap().clone(),
                                    transform: Transform::from_translation(transform.translation),
                                    ..default()
                                },
                                Collider {
                                    size: Vec2::new(16.0, 16.0),
                                    offset: Vec2::ZERO,
                                },
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut query {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn projectile_hit_detection(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Collider, &Projectile)>,
    enemy_query: Query<(Entity, &Transform, &Collider), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (proj_entity, proj_transform, proj_collider, projectile) in &projectile_query {
        for (enemy_entity, enemy_transform, enemy_collider) in &enemy_query {
            if check_collision(proj_transform, proj_collider, enemy_transform, enemy_collider) {
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    amount: projectile.damage,
                    source: DamageSource::Projectile(proj_entity),
                    knockback: Vec2::ZERO,
                });

                commands.entity(proj_entity).despawn();
            }
        }
    }
}
```

#### 11.5 特殊技 - 近接攻撃強化

```rust
fn use_melee_ability(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Player, &CurrentAbility)>,
    registry: Res<AbilityRegistry>,
) {
    for (entity, transform, player, current_ability) in &query {
        if keyboard.just_pressed(KeyCode::KeyX) {
            if let Some(ability_id) = &current_ability.ability_id {
                if let Some(ability) = registry.abilities.get(ability_id) {
                    for special_move in &ability.special_moves {
                        if let SpecialMove::Melee { damage, range, knockback } = special_move {
                            let offset = if player.facing_right { *range } else { -*range };

                            commands.spawn((
                                Hitbox {
                                    damage: *damage,
                                    knockback: Vec2::new(offset * knockback, 100.0),
                                    hit_entities: HashSet::new(),
                                },
                                Collider {
                                    size: Vec2::new(*range, 30.0),
                                    offset: Vec2::new(offset / 2.0, 0.0),
                                },
                                Transform::from_translation(transform.translation),
                                AttackLifetime(Timer::from_seconds(0.3, TimerMode::Once)),
                            ));
                        }
                    }
                }
            }
        }
    }
}
```

#### 11.6 能力UI
- [ ] 現在の能力アイコン表示
- [ ] 能力切り替えメニュー

```rust
#[derive(Component)]
pub struct AbilityIcon;

fn update_ability_icon(
    player_query: Query<&CurrentAbility, (With<Player>, Changed<CurrentAbility>)>,
    mut icon_query: Query<&mut UiImage, With<AbilityIcon>>,
    registry: Res<AbilityRegistry>,
    assets: Res<GameAssets>,
) {
    for current_ability in &player_query {
        for mut icon in &mut icon_query {
            if let Some(ability_id) = &current_ability.ability_id {
                if let Some(ability) = registry.abilities.get(ability_id) {
                    *icon = assets.ability_icons.get(&ability.icon_path).unwrap().clone().into();
                }
            }
        }
    }
}
```

### 成果物
- 複数の能力が実装されている
- 能力に応じた特殊技が使える
- 能力取得時の演出

---

## Phase 12以降の概要

Phase 0-11で基本的なゲームが遊べる状態になりました。Phase 12以降では、より高度な機能、ポリッシュ、最適化を行います。

**Phase 12-20**: 高度なゲームシステム（コンボ、ボス、ワールドマップ等）
**Phase 21-27**: UI/UX改善、アクセシビリティ
**Phase 28-31**: やり込み要素、最適化、ポリッシュ
**Phase 32**: オプション機能（オンライン）

詳細な実装内容は各フェーズで段階的に追加していきます。

---

## まとめと推奨学習パス

### クリティカルパス（最小限の実装）
Phase 0 → 1 → 2 → 4 → 5 → 6 → 7 → 8 → 10 → 15 → 16

これだけで「動くゲーム」になります。

### 完全実装パス
Phase 0-31を順番に実装

推定総開発時間: **6-12ヶ月**（週10-20時間の作業と仮定）

### 学習リソース

#### Bevy
- [Bevy公式ドキュメント](https://bevyengine.org/learn/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)

#### ゲーム設計
- ["Game Programming Patterns" by Robert Nystrom](https://gameprogrammingpatterns.com/)
- ["Game Feel" by Steve Swink](https://www.amazon.com/dp/0123743281)

#### 2Dアクションゲーム分析
- [Celeste Movement Analysis](https://www.youtube.com/watch?v=yorTG9at90g)
- [Hollow Knight Design Analysis](https://www.youtube.com/watch?v=7ITtPRparcしょ)

### 進捗記録の推奨

`docs/implementation_log.md`に以下を記録：
- 実装した日付
- 完了したタスク
- 遭遇した問題と解決方法
- 学んだこと
- 次に取り組むこと

---

## Phase 12-32の詳細実装計画

以下、Phase 12以降の簡易版ロードマップです。各フェーズの詳細実装例は、実際に実装する際に段階的に追加していきます。

---

## Phase 12: 戦闘システムII - コンボ・パリィ（予想時間: 4-5日）

### タスク
- [ ] コンボシステム（連続攻撃）
- [ ] パリィ/カウンターシステム
- [ ] ジャグリング（敵を浮かせる）
- [ ] エアコンボ
- [ ] フィニッシュムーブ

---

## Phase 13: 敵システムII - 高度なAI（予想時間: 4-5日）

### タスク
- [ ] ステートマシンベースのAI
- [ ] 協調攻撃（複数の敵が連携）
- [ ] 学習型AI（プレイヤーの行動パターンに対応）
- [ ] 敵のバリエーション（色違いで強化版）
- [ ] エリートエネミー（ミニボス）

---

## Phase 14: ボスシステム（予想時間: 6-7日）

### タスク
- [ ] ボス専用AI
- [ ] フェーズ制（体力に応じて攻撃パターン変化）
- [ ] ボス専用カメラワーク
- [ ] ボス戦用BGM
- [ ] ボス撃破演出
- [ ] 弱点システム

---

## Phase 15: ワールドマップ（予想時間: 4-5日）

### タスク
- [ ] ステージ選択UI
- [ ] ワールドマップの描画
- [ ] カーソル移動
- [ ] ステージ情報表示
- [ ] アンロック演出

---

## Phase 16: 進行管理 - セーブ/ロード（予想時間: 3-4日）

### タスク
- [ ] セーブデータ構造の実装
- [ ] セーブ処理（RON形式）
- [ ] ロード処理
- [ ] 複数セーブスロット
- [ ] オートセーブ
- [ ] セーブファイル破損対策

---

## Phase 17: UIシステムII - ダメージ数値等（予想時間: 3-4日）

### タスク
- [ ] ダメージ数値表示（ポップアップ）
- [ ] ミニマップ
- [ ] クエストログ
- [ ] インベントリUI
- [ ] ステータス画面

---

## Phase 18: オーディオシステムI（予想時間: 3-4日）

### タスク
- [ ] BGM再生システム
- [ ] 効果音システム
- [ ] ボリューム制御
- [ ] フェードイン/アウト
- [ ] ステージ別BGM
- [ ] ボス戦専用BGM

---

## Phase 19: ビジュアルエフェクトII - シェーダー（予想時間: 5-6日）

### タスク
- [ ] カスタムシェーダーの実装
- [ ] ライティングシステム
- [ ] グロー効果
- [ ] スクリーンスペースエフェクト
- [ ] 天候エフェクト（雨、雪）
- [ ] 水面反射

---

## Phase 20: レベルギミック（予想時間: 4-5日）

### タスク
- [ ] 移動床
- [ ] スイッチと扉
- [ ] ワープゾーン
- [ ] トラップ（トゲ、落とし穴）
- [ ] 破壊可能オブジェクト
- [ ] 隠し部屋

---

## Phase 21: カメラシステムII（予想時間: 3-4日）

### タスク
- [ ] カメラ先読み（プレイヤーの向き先）
- [ ] カメラゾーン（エリアごとの設定）
- [ ] ズームイン/アウト
- [ ] シネマティックカメラ
- [ ] ボス戦カメラロック

---

## Phase 22: 収集要素・実績（予想時間: 3-4日）

### タスク
- [ ] 収集アイテムシステム
- [ ] 実績/トロフィーシステム
- [ ] 進捗トラッキング
- [ ] 報酬システム
- [ ] 隠しアイテム

---

## Phase 23: オーディオシステムII - アダプティブミュージック（予想時間: 4-5日）

### タスク
- [ ] レイヤード音楽（状況に応じて楽器追加）
- [ ] 3D空間オーディオ
- [ ] リバーブ/エコー
- [ ] ダイナミックミュージック（戦闘時に激しく）
- [ ] 音の距離減衰

---

## Phase 24: パフォーマンス最適化I（予想時間: 4-5日）

### タスク
- [ ] オブジェクトプーリング
- [ ] フラスタムカリング
- [ ] LOD (Level of Detail)
- [ ] 非同期ローディング
- [ ] メモリ管理の改善
- [ ] プロファイリングとボトルネック特定

---

## Phase 25: アクセシビリティ（予想時間: 3-4日）

### タスク
- [ ] カラーブラインドモード
- [ ] 字幕表示
- [ ] ボタン配置カスタマイズ
- [ ] 難易度調整オプション
- [ ] アシスト機能（自動回避等）
- [ ] UI拡大オプション

---

## Phase 26: チュートリアルシステム（予想時間: 3-4日）

### タスク
- [ ] チュートリアルダイアログ
- [ ] ガイド矢印
- [ ] 操作説明の表示
- [ ] プレイヤーの進行に応じたヒント
- [ ] スキップ可能なチュートリアル

---

## Phase 27: カットシーン/ダイアログ（予想時間: 4-5日）

### タスク
- [ ] ダイアログシステム
- [ ] キャラクター立ち絵
- [ ] テキスト送り
- [ ] 選択肢システム
- [ ] カットシーン再生
- [ ] スキップ機能

---

## Phase 28: タイムアタック/スコアアタック（予想時間: 2-3日）

### タスク
- [ ] タイムアタックモード
- [ ] ランキングシステム
- [ ] ゴーストデータ
- [ ] スコア計算システム
- [ ] リプレイ保存

---

## Phase 29: リプレイシステム（予想時間: 5-6日）

### タスク
- [ ] 入力記録システム
- [ ] リプレイ再生
- [ ] ゴーストプレイヤー表示
- [ ] リプレイファイル管理
- [ ] 早送り/スロー再生

---

## Phase 30: パフォーマンス最適化II（予想時間: 3-4日）

### タスク
- [ ] 最終的なプロファイリング
- [ ] メモリリーク修正
- [ ] ロード時間短縮
- [ ] フレームレート安定化
- [ ] バッテリー消費最適化（ラップトップ向け）

---

## Phase 31: ポリッシュ（予想時間: 5-7日）

### タスク
- [ ] バグ修正
- [ ] アニメーション微調整
- [ ] サウンドバランス調整
- [ ] UI/UX改善
- [ ] プレイテストフィードバック対応
- [ ] 最終調整

---

## Phase 32 (オプション): オンライン機能（予想時間: 10-15日）

### タスク
- [ ] オンラインリーダーボード
- [ ] ゴーストデータ共有
- [ ] 協力プレイ（2P）
- [ ] 対戦モード
- [ ] フレンド機能
- [ ] サーバー/API実装

**注**: この機能は非常に高度で、別途ネットワークプログラミングの知識が必要です。

---

## 完全実装後の追加アイデア

### DLC/拡張コンテンツ
- 新しいワールド
- 新しいプレイヤーキャラクター
- 新しい能力
- ボスラッシュモード
- 高難易度モード

### コミュニティ機能
- レベルエディター
- カスタムステージ共有
- MODサポート

---

このロードマップに沿って実装を進めることで、現代的で洗練された2Dアクションゲームが完成します。各フェーズは独立しているため、興味のある部分から実装することも可能です。

**重要**: 全てを完璧に実装する必要はありません。自分のペースで、楽しみながら学んでいきましょう！