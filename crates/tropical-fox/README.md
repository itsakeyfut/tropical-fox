# tropical-fox

Tropical Foxゲームのメインバイナリクレート。

## 責任範囲

これは全システムを統合する**ルートクレート**です：

- **アプリケーションエントリーポイント**: ゲーム初期化を行う`main.rs`
- **プラグイン調整**: 全ドメインプラグインを動作するゲームにアセンブル
- **コア設定**: ウィンドウ設定、アセット読み込み、グローバルリソース
- **デバッグ機能**: ホットリロード統合（デバッグビルドのみ）
- **カメラ設定**: メインゲームカメラの初期化

## 設計原則

- **薄い統合レイヤー**: 最小限のロジック、主にプラグイン登録
- **設定優先**: ゲーム開始前にRONファイルから設定を読み込み
- **モジュラーアーキテクチャ**: 各ドメインは独立したプラグイン
- **開発体験の向上**: 高速反復のためのホットリロード

## プロジェクト構成

```
tropical-fox/
├── src/
│   ├── main.rs              # エントリーポイント、プラグインアセンブリ
│   ├── config.rs            # GameSettings読み込み
│   ├── core_plugin.rs       # CorePlugin（カメラ、ウィンドウ）
│   ├── physics_systems.rs   # 物理システム（重力、速度）
│   ├── debug/               # デバッグユーティリティ
│   └── hot_reload_systems.rs  # ホットリロードイベントハンドラ（デバッグのみ）
└── Cargo.toml               # バイナリクレート設定
```

## main関数フロー

```rust
fn main() {
    // 1. 設定を読み込み
    let settings = load_settings_or_default("assets/config/game_settings.ron");

    // 2. プレイヤー設定を読み込んでキャラクターを選択
    let selected_character = load_player_selection();

    // 3. Bevyアプリを構築
    let mut app = App::new();

    // 4. Bevyデフォルトプラグインを追加（カスタム設定付き）
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin { /* ... */ })
        .set(ImagePlugin::default_nearest())  // ピクセルアート
        .set(AssetPlugin { /* ... */ }));      // デバッグでホットリロード

    // 5. ゲームドメインプラグインを追加
    app.add_plugins((
        CorePlugin,
        AnimationPlugin,
        PlayerPlugin,
        CombatPlugin,
        EnemyPlugin,
    ));

    // 6. ホットリロードプラグインを追加（デバッグのみ）
    #[cfg(debug_assertions)]
    app.add_plugins(HotReloadPlugin::<GameSettings>::new(/* ... */));

    // 7. ゲームを実行
    app.run();
}
```

## ドメインプラグイン

メインクレートは以下のプラグインを統合：

1. **CorePlugin** (`core_plugin.rs`)
   - ピクセルパーフェクトスケーリング付きカメラ設定
   - ウィンドウ設定
   - グローバルリソース

2. **AnimationPlugin** (`tropical-fox-animation`から)
   - スプライトアニメーションシステム
   - キャラクターアセット読み込み
   - アニメーション再生

3. **PlayerPlugin** (`tropical-fox-player`から)
   - プレイヤー操作
   - 移動物理
   - プレイヤーエンティティ生成

4. **CombatPlugin** (`tropical-fox-combat`から)
   - ダメージシステム
   - 体力管理
   - 攻撃メカニクス

5. **EnemyPlugin** (`tropical-fox-enemy`から)
   - エネミーAI
   - 敵生成
   - ボスメカニクス

## 設定ファイル

`assets/config/`に配置：

- `game_settings.ron`: ウィンドウサイズ、重力、物理パラメータ
- `players.ron`: プレイヤーキャラクター定義
- `enemies.ron`: 敵タイプ、統計、AI挙動
- `bosses.ron`: ボスキャラクター定義

全設定はデバッグビルドでホットリロード対応。

## ゲーム設定例

```ron
(
    window: (
        title: "Tropical Fox",
        width: 1280.0,
        height: 720.0,
        resizable: true,
    ),
    physics: (
        gravity: 980.0,
        terminal_velocity: 500.0,
    ),
    player: (
        move_speed: 200.0,
        jump_force: 400.0,
        // ... その他の設定
    ),
)
```

## アセットパス解決

バイナリクレートは以下をサポートするアセットパス解決を処理：
- ワークスペースルートからの実行: `cargo run`
- クレートディレクトリからの実行: `cd crates/tropical-fox && cargo run`
- どこからでもコンパイル済みバイナリの実行

パスを順番に試行: `assets`、`../../assets`、`../assets`

## ホットリロードシステム（デバッグのみ）

デバッグビルドでは設定ファイルの変更を監視：

```rust
#[cfg(debug_assertions)]
{
    app.add_plugins(HotReloadPlugin::<GameSettings>::new("config/game_settings.ron"));
    app.add_plugins(HotReloadPlugin::<EnemiesConfig>::new("config/enemies.ron"));
    app.add_plugins(HotReloadPlugin::<PlayersConfig>::new("config/players.ron"));
    app.add_plugins(HotReloadPlugin::<BossesConfig>::new("config/bosses.ron"));

    app.add_systems(Update, (
        apply_game_settings_reload,
        apply_enemies_config_reload,
        apply_players_config_reload,
        apply_bosses_config_reload,
    ));
}
```

RONファイルを修正 → ゲームが自動リロード → 変更が即座に適用

## ビルドコマンド

```bash
# 開発ビルド（ホットリロード付き高速反復）
cargo run --features bevy-dev

# 標準開発ビルド
cargo run

# リリースビルド（最適化）
cargo build --release
```

## デバッグ機能

`#[cfg(debug_assertions)]`でのみ有効：
- 設定ホットリロード
- 詳細リロードログ
- リロードメトリクス追跡
- アセット変更監視

## 依存関係

### 内部クレート
- `tropical-fox-animation`
- `tropical-fox-combat`
- `tropical-fox-common`
- `tropical-fox-enemy`
- `tropical-fox-hot-asset`
- `tropical-fox-player`

### 外部クレート
- `bevy`: ゲームエンジン
- ルート`Cargo.toml`で定義された全ワークスペース依存関係

## ゲームステートマシン

`tropical-fox-common::GameState`で定義：

```
Loading → Title → WorldMap → InGame → (Paused/GameOver)
                                ↓
                    InGameState: StagePlay/BossRoom/StageTransition
```

ほとんどのゲームプレイシステムは`GameState::InGame`でのみ実行。
