# tropical-fox-enemy

Tropical FoxゲームのエネミーAIと挙動システム。

## 責任範囲

このクレートは全ての**敵関連メカニクス**を実装します：

- **エネミーAI**: 複数の行動パターン（待機、巡回、追跡、飛行）
- **ボスシステム**: ユニークなメカニクスを持つ特殊な敵タイプ
- **接触ダメージ**: プレイヤーへの接触ベースダメージ
- **弾幕システム**: 射撃AIを持つ遠距離攻撃
- **敵生成**: 設定ベースの敵インスタンス化
- **アニメーション制御**: 敵のアニメーション状態管理
- **スコアシステム**: 倒した敵のポイント値
- **設定読み込み**: RONベースの敵とボス定義

## 設計原則

- **データ駆動AI**: 敵の挙動はRON設定ファイルで定義
- **モジュラー挙動**: 組み合わせ可能なAIコンポーネント（巡回 + 弾幕射撃）
- **再利用可能システム**: 汎用システムが全敵タイプで動作
- **ステートマシンAI**: 各挙動タイプが独自の状態追跡を持つ

## モジュール構成

```
enemy/
├── components.rs        # Enemy, EnemyAI, ContactDamage, ProjectileShooter
├── config/
│   ├── enemies.rs       # EnemiesConfig, EnemyTypeConfig
│   ├── bosses.rs        # BossesConfig, BossDefinition
│   └── mod.rs           # 設定エクスポート
├── plugin.rs            # EnemyPlugin、AIシステム、生成
└── lib.rs               # 公開エクスポート
```

## 敵設定

**敵RON** (`assets/config/enemies.ron`):
```ron
(
    enemies: {
        "ant": (
            id: "ant",
            name: "Ant",
            animation_config_path: "graphics/characters/enemies/ant/ant_animations.ron",
            stats: (
                health: 30.0,
                move_speed: 80.0,
                damage: 10.0,
                knockback_force: 150.0,
                score_value: 100,
            ),
            collider: (size: (31.0, 31.0), offset: (0.0, 0.0)),
            has_gravity: true,
            ai: (
                behavior: Patrol,
                patrol_distance: 150.0,
                wait_time: 1.0,
            ),
            projectile: (enabled: false),
        ),
    },
)
```

## AI挙動タイプ

### 1. Idle（待機）
- 静止している敵
- 移動なし、ただ存在するだけ

### 2. Patrol（巡回）
- ウェイポイント間を往復
- 各端点で短時間待機
- 距離と待機時間は設定可能

### 3. Chase（追跡）
- 範囲内でプレイヤーを検知
- 検知時にプレイヤーを追跡
- 攻撃範囲内で停止
- 範囲外で一定時間後にアグロ解除

### 4. Flying（飛行）
- 事前定義されたパターンで移動
- 重力適用なし
- パターンタイプ:
  - **SineWave**: 垂直振動を伴う水平移動
  - **Circle**: 原点の周りを円軌道
  - **Hover**: その場で垂直に浮遊
  - **Figure8**: 8の字パターン

## 敵の機能

### 接触ダメージ
- プレイヤーとの衝突でダメージ
- 1回の接触で複数ヒットを防ぐクールダウン
- ノックバックでプレイヤーを敵から押し出す
- プレイヤーの無敵時間を尊重

### 弾幕射撃
- 敵はプレイヤーに向けて弾を発射可能
- ダメージ、速度、発射速度、範囲を設定可能
- 発射時にプレイヤー位置に自動照準
- 弾は寿命切れで消滅

### 死亡とスコア
- 倒されたときにポイント付与
- 死亡エフェクトをトリガー（TODO: パーティクル、ドロップ）
- 死亡時にエンティティを削除

## 敵タイプ

現在実装済み:
- **Ant**: 地上巡回敵
- **Bat**: サインウェーブパターンの飛行敵
- **Bear**: プレイヤーを追跡する追跡敵
- **Piranha**: 静止弾幕射撃敵

ボス対応:
- ボス定義は`bosses.ron`に記載
- 通常敵と同じシステムだがカスタム統計

## システム実行順序

**Updateスケジュール**:
- AI挙動システム（巡回、追跡、飛行）
- `contact_damage_cooldown_system`
- `contact_damage_system`
- `projectile_shooter_system`
- `projectile_movement_system`
- `projectile_collision_system`
- `enemy_animation_controller`
- `enemy_facing_system`

**FixedUpdateスケジュール**:
- `enemy_ground_collision`

**戦闘後**:
- `enemy_death_system` (combatクレートの`death_system`後に実行)

## 使用例

```rust
use tropical_fox_enemy::{EnemyPlugin, spawn_enemy};

// プラグイン経由での自動生成
App::new()
    .add_plugins(EnemyPlugin)
    .run();

// 手動敵生成
spawn_enemy(
    &mut commands,
    "ant",
    Vec2::new(100.0, 50.0),
    &enemy_config,
    Some(&character_assets),
);
```

## 主要コンポーネント

- `Enemy`: enemy_typeと向きを持つマーカーコンポーネント
- `EnemyStats`: ダメージ、速度、体力、スコア値
- `EnemyAI`: AI挙動状態のEnum（Idle/Patrol/Chase/Flying）
- `ContactDamage`: クールダウン付き接触ダメージ
- `ProjectileShooter`: 遠距離攻撃システム
- `EnemyProjectile`: 弾エンティティコンポーネント

### AIコンポーネント（内部）
- `PatrolAI`: ウェイポイント追跡と待機タイマー
- `ChaseAI`: 検知範囲、アグロ状態
- `FlyingAI`: パターンタイプと移動時間
- `FlyingPattern`: サインウェーブ、円、ホバー、8の字

## アニメーション統合

敵アニメーションはAI状態で制御:
- Idle AI → "idle"アニメーション
- Patrol AI → "idle"（停止時）または"run"（移動時）
- Chase AI → "idle"（待機時）または"run"（追跡時）
- Flying AI → "fly"アニメーション（"idle"にフォールバック）

## スコアリソース

`Score`リソースで総ポイントを追跡:
```rust
fn check_score(score: Res<Score>) {
    info!("現在のスコア: {}", score.value);
}
```

## 依存関係

- `tropical-fox-common`: 共有型、プレイヤー参照、ゲーム状態
- `tropical-fox-animation`: アニメーションコントローラーと設定読み込み
- `tropical-fox-combat`: 体力、ダメージイベント、死亡イベント
- `bevy`: コアECS
- `serde`/`ron`: 設定読み込み
