# tropical-fox-hot-asset

Bevy統合付きRON設定ファイルのホットリロードシステム。

## 責任範囲

このクレートは**開発時ホットリロード機能**を提供します：

- **ファイル監視**: `notify`を使用してRONファイルの変更を監視
- **自動リロード**: ファイル変更を検知して自動的にアセットをリロード
- **Bevy統合**: 簡単に統合できる`HotReloadPlugin`を提供
- **イベントシステム**: 設定変更時に`AssetReloaded<T>`イベントを発行
- **デバウンス**: 連続ファイル保存によるリロード嵐を防止
- **メトリクス追跡**: オプションでリロード履歴とパフォーマンス統計
- **エラー耐性**: 解析失敗時は前回の有効な値を保持

## 設計原則

- **型安全**: `Asset + TypePath + for<'de> Deserialize<'de>`を実装する任意の型に対応
- **リリースビルドでランタイムコストゼロ**: デバッグビルドでのみ有効
- **開発者体験優先**: ゲームを再起動せずに設定を反復可能
- **グレースフルデグラデーション**: 無効なファイルでもクラッシュせずエラーログを出力

## モジュール構成

```
hot-asset/
├── core/
│   ├── watcher.rs        # notifyによるファイルシステム監視
│   ├── debounce.rs       # デバウンスロジック
│   ├── metrics.rs        # リロード統計
│   └── path_resolver.rs  # アセットパス解決
└── bevy_adapter/
    ├── plugin.rs         # HotReloadPlugin
    ├── resources.rs      # HotAssetHandle<T>
    ├── events.rs         # AssetReloaded<T>
    ├── systems.rs        # ファイル監視ポーリング
    └── loader.rs         # RONデシリアライズ
```

## 使用例

```rust
use bevy::prelude::*;
use tropical_fox_hot_asset::HotReloadPlugin;

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
struct GameSettings {
    gravity: f32,
    player_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(
            HotReloadPlugin::<GameSettings>::new("config/game_settings.ron")
                .with_verbose(true)
                .with_metrics(true)
        )
        .add_systems(Update, handle_settings_reload)
        .run();
}

fn handle_settings_reload(
    mut reload_events: MessageReader<AssetReloaded<GameSettings>>,
    handle: Res<HotAssetHandle<GameSettings>>,
) {
    for event in reload_events.read() {
        if let Some(settings) = handle.get() {
            info!("設定がリロードされました: gravity = {}", settings.gravity);
            // 新しい設定をゲームに適用...
        }
    }
}
```

## 主要機能

### ファイル監視
- クロスプラットフォームのファイルシステムイベントに`notify`クレートを使用
- `assets/`ディレクトリとサブディレクトリを監視
- 指定された設定ファイルのみにイベントをフィルタリング

### デバウンス
- 単一の保存操作からの複数リロードトリガーを防止
- 設定可能なデバウンスウィンドウ（デフォルト: 100ms）
- 連続した高速ファイル書き込みを優雅に処理

### メトリクス（オプション）
```rust
let plugin = HotReloadPlugin::<T>::new(path).with_metrics(true);
// ReloadMetricsリソース経由でメトリクスにアクセス
```
- リロード回数、成功/失敗率を追跡
- タイムスタンプ付きリロード履歴を記録
- 設定反復のデバッグに有用

### エラーハンドリング
- 解析エラーはログに記録されるがゲームはクラッシュしない
- 前回の有効な設定が使用中のまま
- ファイルパスコンテキスト付きの明確なエラーメッセージ

## Tropical Foxでの統合

`main.rs`で以下の設定をホットリロード：
- `GameSettings` (ウィンドウ、物理、プレイヤー移動)
- `PlayersConfig` (キャラクター定義)
- `EnemiesConfig` (敵の統計とAI)
- `BossesConfig` (ボス定義)

## 設定ファイル形式

RON（Rusty Object Notation）ファイル例：
```ron
(
    gravity: 980.0,
    player_speed: 200.0,
)
```

## パフォーマンス

- ファイル監視は別スレッドで非同期実行
- Bevyシステムはフレームごとに1回チャンネルをポーリング（最小限のオーバーヘッド）
- デバウンスが不要なリロード試行を削減
- リリースビルドではランタイムコストゼロ（`#[cfg(debug_assertions)]`で制御）

## 依存関係

- `bevy`: コアECSとアセットシステム
- `notify`: クロスプラットフォームファイルシステム監視
- `crossbeam-channel`: イベント受け渡しのためのスレッドセーフチャンネル
- `ron`: RONデシリアライズ
- `serde`: シリアライゼーションフレームワーク
- `thiserror`: エラー型定義
