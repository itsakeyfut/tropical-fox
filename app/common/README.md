# tropical-fox-common

Tropical Foxゲームの全クレートで共有される共通型とユーティリティ。

## 責任範囲

このクレートはゲーム全体の**基盤レイヤー**として、以下を提供します：

- **ゲーム状態管理**: `GameState`と`InGameState`によるステートマシン
- **共有ECSコンポーネント**: 物理系（`Velocity`、`Gravity`、`Collider`）、プレイヤー/敵マーカー、地面検知
- **リソース定義**: キャラクターアセット、テクスチャアトラス、共有ゲームリソース
- **イベント定義**: 複数ドメインで使用される共通イベント

## 設計原則

- **依存関係のないドメイン層**: 他のゲームクレートに依存しない
- **最小限の外部依存**: Bevyコア、serde、thiserrorのみに依存
- **共有契約**: 特化したドメインクレート間のインターフェース契約として機能

## モジュール構成

```
common/
├── components/     # 共有ECSコンポーネント
│   ├── animation.rs
│   ├── enemy.rs
│   └── mod.rs
├── events/         # 共通イベント型
├── game_state.rs   # ステートマシン定義
└── resources/      # 共有リソース
```

## 使用例

```rust
use tropical_fox_common::{GameState, Player, Velocity, Gravity};

fn my_system(query: Query<&Velocity, With<Player>>) {
    // 任意のドメインで共通コンポーネントを使用
}
```

## 主要コンポーネント

- `Player`、`Enemy`、`Ground`、`Wall`: エンティティマーカー
- `Velocity`、`Gravity`、`Collider`: 物理コンポーネント
- `GroundDetection`: プラットフォーム衝突状態
- `PlayerStats`: プレイヤー設定データ
- `CharacterAssets`: 共有テクスチャアトラスリソース

## 依存クレート

このクレートは全ての特化ドメインクレートからインポートされます：
- `tropical-fox-animation`
- `tropical-fox-combat`
- `tropical-fox-player`
- `tropical-fox-enemy`
