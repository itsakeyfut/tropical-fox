# tropical-fox-animation

Tropical Foxゲームのフレームベーススプライトアニメーションシステム。

## 責任範囲

このクレートはゲームの**アニメーションインフラ**を提供します：

- **アニメーション再生**: テクスチャアトラスを使用したフレームごとのスプライトアニメーション
- **アニメーションコントローラー**: アニメーションクリップ（idle、run、jumpなど）を管理するステートマシン
- **設定読み込み**: アニメーション定義のRONファイル解析
- **アニメーションイベント**: フレーム固有のイベント（効果音、VFX生成）
- **キャラクター非依存**: プレイヤー、敵、NPC、ボス全てに対応

## 設計原則

- **データ駆動**: アニメーションはRONファイル（`graphics/characters/*/animations.ron`）で定義
- **汎用システム**: 単一のアニメーションシステムが全キャラクタータイプに対応
- **グレースフルフォールバック**: 無効な設定はハードコードされたデフォルトにフォールバック
- **ECSベース**: Bevyコンポーネント（`AnimationController`、`AnimationState`）を使用

## モジュール構成

```
animation/
├── components.rs   # AnimationController, AnimationState, AnimationClip
├── config.rs       # RONファイル読み込みと解析
├── systems.rs      # update_animations, player_animation_controller
├── plugin.rs       # AnimationPlugin登録
└── lib.rs          # 公開エクスポート
```

## 使用例

**RON設定** (`fox_animations.ron`):
```ron
(
    spritesheet_path: "graphics/characters/players/fox/spritesheets/fox.png",
    sprite_size: (32, 32),
    columns: 6,
    rows: 12,
    clips: {
        "idle": (first: 0, last: 3, fps: 6.5),
        "run": (first: 6, last: 11, fps: 6.5),
        "jump": (first: 30, last: 30, fps: 1.0),
    },
    default_animation: "idle",
)
```

**コード統合**:
```rust
use tropical_fox_animation::{AnimationController, AnimationState, load_animation_config};

// RONファイルから読み込み
let config = load_animation_config("assets/graphics/characters/fox/fox_animations.ron")?;
let controller = AnimationController::try_from(config)?;
let (controller, state) = controller.with_initial_state(true);

// エンティティに追加
commands.spawn((
    controller,
    state,
    Sprite { /* ... */ },
));
```

## 主要システム

- `update_animations` (FixedUpdate): 時間に基づいてアニメーションフレームを進める
- `player_animation_controller` (FixedUpdate): 移動状態に基づいてプレイヤーアニメーションを選択
- `process_animation_events` (Update): フレーム固有のイベントをトリガー

## 依存関係

- `tropical-fox-common`: 共有型とゲーム状態
- `benimator`: 未使用（カスタム実装）
- `bevy`: コアECSとアセットシステム
- `ron`: 設定ファイル解析
