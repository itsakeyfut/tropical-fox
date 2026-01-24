# tropical-fox-player

Tropical Foxゲームのプレイヤー操作システム。

## 責任範囲

このクレートは全ての**プレイヤー関連メカニクス**を処理します：

- **入力処理**: 移動、ジャンプ、ダッシュのキーボード操作
- **移動物理**: 加速と減速を伴う水平移動
- **ジャンプメカニクス**: 可変ジャンプ高度、コヨーテタイム、ジャンプバッファリング
- **壁メカニクス**: 壁滑り、壁ジャンプ
- **ダッシュ能力**: クールダウンと慣性保存付き空中ダッシュ
- **地面検知**: プラットフォーム衝突と接地状態追跡
- **スプライト制御**: 向きに基づく方向別スプライト反転
- **キャラクター選択**: 設定読み込み付きマルチキャラクター対応

## 設計原則

- **タイトで応答性の高い操作**: CelesteとHollow Knightにインスパイア
- **データ駆動設定**: プレイヤー統計はRONファイルで定義
- **物理ベース移動**: 自然な感触のために速度と重力を使用
- **状態ベースアニメーション**: 移動状態（idle/run/jump/fall）によるアニメーション駆動

## モジュール構成

```
player/
├── config.rs       # PlayersConfig, SelectedCharacterリソース
├── systems.rs      # 移動、ジャンプ、ダッシュ、衝突システム
├── plugin.rs       # PlayerPlugin、プレイヤー生成
└── lib.rs          # 公開エクスポート
```

## プレイヤー設定

**RONファイル** (`assets/config/players.ron`):
```ron
(
    players: {
        "fox": (
            id: "fox",
            name: "Fox",
            animation_config_path: "graphics/characters/players/fox/fox_animations.ron",
            description: "機敏なキツネキャラクター",
        ),
    },
    default_player: "fox",
)
```

**統計** (`tropical-fox-common::PlayerStats`で定義):
```rust
PlayerStats {
    move_speed: 200.0,
    jump_force: 400.0,
    wall_jump_force_x: 300.0,
    wall_jump_force_y: 350.0,
    dash_speed: 500.0,
    dash_duration: 0.2,
    // ...
}
```

## 移動メカニクス

### 水平移動
- スムーズな加速と減速
- 地面と空中での制御
- 方向転換処理

### ジャンプシステム
- **通常ジャンプ**: スペースキーでジャンプ（接地が必要）
- **可変高度**: 長押しで高いジャンプ、早めのリリースで小ジャンプ
- **コヨーテタイム**: プラットフォームを離れた後の短い猶予期間
- **ジャンプバッファリング**: 着地前のジャンプ入力をキューイング

### 壁メカニクス
- **壁検知**: 衝突を使用して左右の壁を検知
- **壁滑り**: 壁に触れて壁に向かって移動している間、重力を軽減
- **壁ジャンプ**: 水平ブースト付きで壁から離れてジャンプ

### ダッシュ能力
- **空中ダッシュ**: 空中でShiftキーを押す
- **クールダウン**: 空中で1回のダッシュ（着地でリセット）
- **持続時間**: 高い水平速度での固定持続時間
- **慣性**: ダッシュ終了後も一部の速度を保持

## システム実行順序

**Updateスケジュール**（入力応答性のため）:
- `player_horizontal_movement`
- `player_jump`
- `wall_jump`
- `variable_jump_height`
- `player_dash`
- `update_dash`
- `wall_slide`

**FixedUpdateスケジュール**（物理精度のため）:
- `ground_collision`
- `wall_collision`
- `flip_sprite_by_facing`
- `player_animation_controller` (animationクレートから)

## 使用例

```rust
use tropical_fox_player::{PlayerPlugin, SelectedCharacter};

fn main() {
    App::new()
        .insert_resource(SelectedCharacter::new("fox"))
        .add_plugins(PlayerPlugin)
        .run();
}
```

## 主要コンポーネント

`tropical-fox-common`から:
- `Player`: マーカーコンポーネント
- `PlayerStats`: 設定値
- `Velocity`: 物理速度
- `Gravity`: 重力適用
- `GroundDetection`: プラットフォーム衝突状態
- `Collider`: 衝突ボックス

プレイヤー固有:
- `SelectedCharacter`: キャラクター選択用リソース

## アニメーション統合

プレイヤーアニメーションは移動状態で制御：
- `idle`: 接地 + 水平移動なし
- `run`: 接地 + 水平移動中
- `jump`: 空中 + 上昇中（velocity.y > 50）
- `fall`: 空中 + 下降中（velocity.y <= 50）

追加アニメーション（未トリガー）:
- `climb`、`crouch`、`hurt`、`dizzy`、`roll`、`look_up`、`win`

## 依存関係

- `tropical-fox-common`: 共有型とゲーム状態
- `tropical-fox-animation`: アニメーションコントローラーとシステム
- `tropical-fox-combat`: 体力、ライフ、攻撃コンポーネント
- `bevy`: コアECSと入力処理
- `serde`/`ron`: 設定読み込み
