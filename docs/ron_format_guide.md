# RON設定ファイルリファレンス

このドキュメントはTropical Foxプロジェクトで使用するRON（Rusty Object Notation）設定ファイルの詳細リファレンスです。

---

## 目次

1. [RONとは](#ronとは)
2. [基本構文](#基本構文)
3. [キャラクター設定](#キャラクター設定)
4. [敵設定](#敵設定)
5. [レベル設定](#レベル設定)
6. [ゲーム設定](#ゲーム設定)
7. [よくあるエラー](#よくあるエラー)

---

## RONとは

RON (Rusty Object Notation) はRust向けのデータシリアライゼーションフォーマットです。

**特徴**:
- Rustの構文に似ている
- 型安全
- コメントが書ける
- 人間が読み書きしやすい

**基本的なファイル構造**:
```ron
(
    field1: value1,
    field2: value2,
    nested: (
        sub_field: value3,
    ),
)
```

---

## 基本構文

### データ型

```ron
// 数値
integer: 42,
float: 3.14,
negative: -10,

// 文字列
name: "Fox",
path: "assets/graphics/player.png",

// ブール値
enabled: true,
visible: false,

// タプル（座標など）
position: (x: 100.0, y: 200.0),
size: (width: 32.0, height: 32.0),

// リスト
items: [1, 2, 3, 4],
names: ["apple", "banana", "cherry"],

// マップ（辞書）
animations: {
    "idle": (first: 0, last: 3),
    "run": (first: 4, last: 7),
},

// Enum（列挙型）
behavior: Patrol,
state: Flying,

// Option（オプショナル）
optional_field: Some(42),
nullable_field: None,
```

### コメント

```ron
// 1行コメント

/*
  複数行コメント
  このように書けます
*/

(
    // フィールドにコメントを付けられる
    speed: 150.0,  // ピクセル/秒
)
```

---

## キャラクター設定

### ファイルパス
`assets/config/characters/{character_name}.ron`

### 完全な例（Fox）

```ron
(
    // ===== 基本パラメータ =====
    stats: (
        move_speed: 150.0,        // ピクセル/秒 - 水平移動速度
        jump_force: 300.0,        // ピクセル/秒 - ジャンプの初速
        max_jumps: 2,             // 最大ジャンプ回数（1=通常、2=2段ジャンプ）
        dash_speed: 250.0,        // ピクセル/秒 - ダッシュ速度
        invincibility_time: 1.5,  // 秒 - 被ダメージ後の無敵時間
    ),

    // ===== 物理パラメータ =====
    physics: (
        gravity_scale: 1.0,       // 重力の影響度（0.0=無重力、1.0=標準、2.0=2倍）
        friction: 0.8,            // 地面との摩擦係数（0.0=滑る、1.0=止まる）
        mass: 1.0,                // 質量（衝突時のノックバックに影響）
    ),

    // ===== 体力 =====
    health: (
        max: 100.0,               // 最大体力
        starting: 100.0,          // ゲーム開始時の体力
    ),

    // ===== 衝突判定 =====
    collider: (
        size: (width: 24.0, height: 28.0),  // 当たり判定のサイズ（ピクセル）
        offset: (x: 0.0, y: -2.0),          // スプライト中心からのオフセット
    ),

    // ===== アニメーション設定 =====
    animations: (
        // スプライトシートのパス
        spritesheet_path: "graphics/characters/players/fox/spritesheets/fox.png",

        // 1つのスプライトのサイズ
        sprite_size: (width: 32.0, height: 32.0),

        // スプライトシートのグリッド構造
        columns: 6,               // 横に何枚並んでいるか
        rows: 10,                 // 縦に何枚並んでいるか

        // アニメーションクリップの定義
        clips: {
            "idle": (
                first: 0,         // 開始フレーム番号（0始まり）
                last: 3,          // 終了フレーム番号
                fps: 8.0,         // フレームレート（1秒あたりのフレーム数）
            ),
            "run": (
                first: 6,
                last: 11,
                fps: 12.0,
            ),
            "jump": (
                first: 12,
                last: 15,
                fps: 10.0,
            ),
            "crouch": (
                first: 18,
                last: 20,
                fps: 6.0,
            ),
            "hurt": (
                first: 24,
                last: 25,
                fps: 8.0,
            ),
            "dizzy": (
                first: 48,
                last: 53,
                fps: 10.0,
            ),
            "roll": (
                first: 54,
                last: 57,
                fps: 15.0,
            ),
        },

        // デフォルトで再生するアニメーション
        default_animation: "idle",
    ),
)
```

### フィールド説明

#### `stats`
| フィールド | 型 | 推奨範囲 | 説明 |
|-----------|-----|---------|------|
| `move_speed` | f32 | 100-250 | 歩き/走りの速度 |
| `jump_force` | f32 | 200-400 | ジャンプの強さ |
| `max_jumps` | u8 | 1-3 | 連続ジャンプ回数 |
| `dash_speed` | f32 | 200-400 | ダッシュ/ロール時の速度 |
| `invincibility_time` | f32 | 1.0-2.5 | 無敵時間（秒） |

#### `physics`
| フィールド | 型 | 推奨範囲 | 説明 |
|-----------|-----|---------|------|
| `gravity_scale` | f32 | 0.5-2.0 | 重力の強さ（1.0が標準） |
| `friction` | f32 | 0.0-1.0 | 摩擦（0で滑る、1で即停止） |
| `mass` | f32 | 0.5-2.0 | 重さ（ノックバックに影響） |

#### `animations.clips`
- **first/last**: スプライトシートの左上から数えたフレーム番号（0始まり）
- **fps**: 1秒あたりのフレーム数（高いほど速い）

**フレーム番号の計算例**:
```
スプライトシートが6列 x 10行の場合:
0  1  2  3  4  5
6  7  8  9  10 11
12 13 14 15 16 17
...

"idle"が0-3なら: 1行目の左から4枚
"run"が6-11なら: 2行目の全て
```

---

## 敵設定

### ファイルパス
`assets/config/enemies/{enemy_name}.ron`

### 完全な例（Bat - 飛行敵）

```ron
(
    // 敵の表示名
    name: "Bat",

    // ===== 基本ステータス =====
    stats: (
        move_speed: 80.0,         // ピクセル/秒 - 移動速度
        damage: 10.0,             // プレイヤーに与えるダメージ
        score_value: 50,          // 倒した時のスコア
    ),

    // ===== 体力 =====
    health: (
        max: 20.0,                // 最大体力（倒すまでに必要なダメージ）
    ),

    // ===== AI設定 =====
    ai: (
        behavior: Flying,         // AI行動パターン（Patrol, Chase, Flee, Stationary, Flying）
        detection_range: 200.0,   // プレイヤーを検知する距離（ピクセル）
        attack_range: 40.0,       // 攻撃を開始する距離（ピクセル）
        patrol_speed: 60.0,       // 巡回時の速度（chase時はstats.move_speedを使用）
    ),

    // ===== 物理設定 =====
    physics: (
        gravity_scale: 0.0,       // 0.0なら空中に浮く（飛行敵用）
        friction: 0.9,
        mass: 0.5,                // 軽い敵
    ),

    // ===== 衝突判定 =====
    collider: (
        size: (width: 20.0, height: 16.0),
        offset: (x: 0.0, y: 0.0),
    ),

    // ===== アニメーション =====
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

### 地上敵の例（Slug）

```ron
(
    name: "Slug",

    stats: (
        move_speed: 40.0,         // 遅い敵
        damage: 5.0,
        score_value: 30,
    ),

    health: (
        max: 15.0,
    ),

    ai: (
        behavior: Patrol,         // 巡回のみ
        detection_range: 100.0,
        attack_range: 30.0,
        patrol_speed: 40.0,
    ),

    physics: (
        gravity_scale: 1.0,       // 地上敵は重力に従う
        friction: 0.7,
        mass: 1.0,
    ),

    collider: (
        size: (width: 24.0, height: 16.0),
        offset: (x: 0.0, y: -4.0),
    ),

    animations: (
        spritesheet_path: "graphics/characters/enemies/slug/spritesheets/slug.png",
        sprite_size: (width: 32.0, height: 32.0),
        columns: 2,
        rows: 1,

        clips: {
            "walk": (first: 0, last: 1, fps: 4.0),
        },

        default_animation: "walk",
    ),
)
```

### AI行動パターン

| Behavior | 説明 | 用途 |
|----------|------|------|
| `Patrol` | 決まったルートを巡回 | 地上の雑魚敵 |
| `Chase` | プレイヤーを検知したら追いかける | 積極的な敵 |
| `Flee` | プレイヤーから逃げる | 臆病な敵 |
| `Stationary` | その場に留まり、範囲内で攻撃 | 固定砲台 |
| `Flying` | 上下左右に飛行 | コウモリ、鳥 |

---

## レベル設定

### ファイルパス
`assets/config/levels/level_{number}.ron`

### 完全な例

```ron
(
    // レベル名
    name: "Sunny Land - Level 1",

    // ===== 環境設定 =====
    environment: (
        // タイルマップ画像
        tilemap: "graphics/environments/sunny_land/tilesets/tileset.png",
        tile_size: 16.0,          // 1タイルのサイズ（ピクセル）

        // 背景レイヤー（後ろから前へ）
        background_layers: [
            "graphics/environments/sunny_land/layers/back.png",
            "graphics/environments/sunny_land/layers/middle.png",
        ],

        // 視差スクロール速度（0.0=固定、1.0=カメラと同速）
        parallax_speeds: [0.3, 0.6],
    ),

    // ===== BGM =====
    music: "audio/music/platformer_level03.ogg",

    // ===== プレイヤー出現位置 =====
    player_spawn: (x: 100.0, y: 200.0),

    // ===== 敵の配置 =====
    enemies: [
        (
            type: "bat",                      // 敵のタイプ（config/enemies/bat.ron）
            position: (x: 300.0, y: 150.0),   // 出現座標
        ),
        (
            type: "slug",
            position: (x: 450.0, y: 250.0),
        ),
        (
            type: "frog",
            position: (x: 600.0, y: 250.0),
        ),
    ],

    // ===== 収集アイテムの配置 =====
    collectibles: [
        (
            type: "cherry",                   // アイテムタイプ
            position: (x: 200.0, y: 180.0),
        ),
        (
            type: "gem",
            position: (x: 350.0, y: 120.0),
        ),
        (
            type: "star",
            position: (x: 500.0, y: 200.0),
        ),
    ],

    // ===== カメラ設定 =====
    camera: (
        follow_speed: 3.0,        // プレイヤー追従の滑らかさ（高いほど速い）
        bounds: (
            min: (x: 0.0, y: 0.0),
            max: (x: 1920.0, y: 1080.0),
        ),
    ),
)
```

### 収集アイテムの種類

| type | 説明 | スコア例 |
|------|------|---------|
| `"cherry"` | 小アイテム | 10 |
| `"gem"` | 中アイテム | 50 |
| `"star"` | 大アイテム | 100 |
| `"acorn"` | 特殊アイテム | 30 |
| `"carrot"` | 体力回復 | 20 + HP回復 |

---

## ゲーム設定

### ファイルパス
`assets/config/game_settings.ron`

### 完全な例

```ron
(
    // ===== ウィンドウ設定 =====
    window: (
        title: "Tropical Fox",    // ウィンドウタイトル
        width: 1280,              // ウィンドウ幅（ピクセル）
        height: 720,              // ウィンドウ高さ（ピクセル）
        resizable: true,          // サイズ変更可能か
        vsync: true,              // 垂直同期（ティアリング防止）
    ),

    // ===== グラフィック設定 =====
    graphics: (
        pixel_perfect: true,      // ピクセルパーフェクト描画
        scaling_mode: NearestNeighbor,  // 拡大時の補間方法
        target_resolution: (
            width: 320,           // 内部解像度（ドット絵風にするため低解像度）
            height: 180,
        ),
    ),

    // ===== オーディオ設定 =====
    audio: (
        master_volume: 0.7,       // マスター音量（0.0-1.0）
        music_volume: 0.6,        // BGM音量（0.0-1.0）
        sfx_volume: 0.8,          // 効果音音量（0.0-1.0）
    ),

    // ===== ゲームプレイ設定 =====
    gameplay: (
        gravity: 500.0,           // 重力加速度（ピクセル/秒^2）
        terminal_velocity: 400.0, // 最大落下速度（ピクセル/秒）
    ),
)
```

### scaling_modeの選択肢

| モード | 説明 | 用途 |
|--------|------|------|
| `NearestNeighbor` | 最近傍法（ドット絵がくっきり） | ピクセルアート |
| `Linear` | 線形補間（滑らか） | 通常のグラフィック |

---

## よくあるエラー

### 1. カンマ忘れ

**エラー例**:
```ron
(
    speed: 150.0  // ← カンマがない
    jump: 300.0,
)
```

**正しい**:
```ron
(
    speed: 150.0,  // ← カンマを追加
    jump: 300.0,
)
```

### 2. 引用符の間違い

**エラー例**:
```ron
name: 'Fox',  // シングルクォートは使えない
```

**正しい**:
```ron
name: "Fox",  // ダブルクォートを使う
```

### 3. タプルの書き方

**エラー例**:
```ron
position: [100.0, 200.0],  // リストになっている
```

**正しい**:
```ron
position: (x: 100.0, y: 200.0),  // 名前付きタプル
```

### 4. パスの区切り文字

**エラー例**:
```ron
path: "graphics\\player\\fox.png",  // Windowsスタイル（エスケープが必要）
```

**正しい**:
```ron
path: "graphics/player/fox.png",  // Unixスタイル（推奨）
```

### 5. 小数点忘れ

**エラー例**:
```ron
speed: 150,  // 整数になる
```

**正しい**:
```ron
speed: 150.0,  // 浮動小数点数
```

---

## 検証ツール

RONファイルが正しいかチェックする方法:

### コマンドラインツール

```bash
# ronコマンドをインストール
cargo install --features bin ron

# ファイルを検証
ron check assets/config/characters/fox.ron
```

### Rustコードで検証

```rust
use ron::de::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestConfig {
    speed: f32,
}

fn validate_ron_file(path: &str) -> Result<(), String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read: {}", e))?;

    from_str::<TestConfig>(&content)
        .map_err(|e| format!("Invalid RON: {}", e))?;

    Ok(())
}
```

---

## まとめ

### RONファイル作成のチェックリスト

- [ ] 全てのフィールドにカンマがある
- [ ] 文字列はダブルクォート
- [ ] パスは`/`（スラッシュ）区切り
- [ ] 数値に適切な型（整数 or 浮動小数点）
- [ ] 括弧の対応が正しい
- [ ] コメントで単位を明記

### デバッグのコツ

1. **最小限から始める**: 基本フィールドだけで動作確認
2. **既存ファイルをコピー**: 動作するファイルを元に編集
3. **エラーメッセージを読む**: RONパーサーは行番号を教えてくれる
4. **段階的に追加**: 1つずつフィールドを増やして動作確認

---

**次のステップ**: 実際に`assets/config/`ディレクトリを作成し、サンプルRONファイルを配置してみましょう！
