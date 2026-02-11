# tropical-fox-combat

Tropical Foxゲームの戦闘メカニクスとダメージシステム。

## 責任範囲

このクレートは全ての**戦闘関連メカニクス**を実装します：

- **体力管理**: HP追跡、ダメージ適用、死亡検知
- **攻撃システム**: ヒットボックス生成、攻撃クールダウン、衝突検知
- **ダメージ処理**: ダメージイベント、無敵時間、ダメージ点滅
- **ノックバック物理**: ヒット時の力の適用
- **視覚フィードバック**: ヒットストップ（フリーズフレーム）、画面振動
- **ライフシステム**: プレイヤーのライフ数とリスポーンメカニクス

## 設計原則

- **イベント駆動**: 疎結合のためにBevyメッセージ（`DamageEvent`、`DeathEvent`）を使用
- **順序付き実行**: システムは慎重に順序付けられたフェーズで実行（入力→検知→ダメージ→エフェクト→死亡）
- **普遍的適用**: プレイヤーと敵の両方のダメージに対応
- **関心の分離**: 戦闘ロジックは移動/AIから独立

## モジュール構成

```
combat/
├── health.rs       # Health, Lives, PlayerHealth, EnemyHealthコンポーネント
├── damage.rs       # DamageEvent処理、無敵時間
├── attack.rs       # 攻撃ヒットボックス、クールダウン、衝突検知
├── effects.rs      # ヒットストップ、画面振動、ダメージ点滅
└── lib.rs          # CombatPlugin、イベント定義
```

## 戦闘フロー

```
プレイヤー入力（攻撃）
    ↓
攻撃クールダウンチェック
    ↓
攻撃ヒットボックス生成
    ↓
攻撃衝突検知
    ↓
DamageEventを送信
    ↓
ダメージ + ノックバック + 無敵時間を適用
    ↓
視覚エフェクト（点滅、ヒットストップ、画面振動）
    ↓
死亡チェック → DeathEventを送信
    ↓
エンティティ削除 / プレイヤーリスポーン
```

## 使用例

```rust
use tropical_fox_combat::{Health, DamageEvent, AttackCooldown};

// エンティティに戦闘コンポーネントを追加
commands.spawn((
    Health::new(100.0),
    PlayerHealth,
    Lives::new(3),
    AttackCooldown::default(),
));

// ダメージイベントを送信
damage_events.write(DamageEvent {
    target: enemy_entity,
    damage: 25.0,
    knockback: Vec2::new(200.0, 100.0),
    attacker: Some(player_entity),
});
```

## 主要コンポーネント

- `Health`: max_health追跡付きHP値
- `PlayerHealth` / `EnemyHealth`: タイプ固有の動作のためのマーカーコンポーネント
- `Lives`: プレイヤーのライフカウンター
- `AttackCooldown`: 攻撃スパム防止
- `Invincibility`: 一時的な無敵状態（i-frame）
- `DamageFlash`: 視覚フィードバックタイマー

## 主要イベント

- `DamageEvent`: ダメージが与えられたときにトリガー
- `DeathEvent`: エンティティが死亡したときにトリガー
- `HitStopEvent`: 衝撃時の短い一時停止をリクエスト
- `ScreenShakeEvent`: カメラ振動エフェクトをリクエスト

## システム実行順序

全ての戦闘システムは`Update`スケジュールで実行され、この順序でチェーン：
1. `update_attack_cooldown`
2. `player_attack_input`
3. `attack_collision_system`
4. `attack_lifetime_system`
5. `damage_system`
6. `hitstop_system`
7. `screen_shake_system`
8. `invincibility_system`
9. `damage_flash_system`
10. `knockback_system`
11. `death_system`
12. `player_death_system`

## 依存関係

- `tropical-fox-common`: 共有型とゲーム状態
- `bevy`: コアECS
- `bevy_rapier2d`: 物理統合（最小限の使用）
- `rand`: エフェクト用の乱数生成
