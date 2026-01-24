# TODO: Level System Setup

## LDtkエディタでの設定が必要な項目

### 現在の状況

- ✅ `assets/levels/test.ldtk` - LDtkプロジェクトファイル作成済み
- ✅ `assets/levels/cavernas.png` - タイルセット画像準備済み
- ⚠️ IntGrid値とエンティティの設定が未完了

### 必要な設定作業

## 1. IntGrid値の設定

LDtkエディタで `test.ldtk` を開き、以下を設定：

### 手順

1. LDtkエディタで `assets/levels/test.ldtk` を開く
2. 左パネルで **「Layers」** タブを選択
3. **「IntGrid_layer」** をクリックして選択
4. 右側のレイヤー設定パネルで **IntGrid values** セクションを探す
5. 既存の値を以下のように変更・追加：

#### IntGrid値の定義

| Value | Identifier | 用途 | 推奨色 | 説明 |
|-------|-----------|------|--------|------|
| **1** | `Ground` | 地面 | `#7F5417`（茶色） | プレイヤーが上に立てる地面タイル |
| **2** | `Wall` | 壁 | `#808080`（グレー） | プレイヤーが通過できない壁 |
| **3** | `OneWayPlatform` | 一方通行 | `#4169E1`（青） | 下から飛び上がれるプラットフォーム |

### 設定方法の詳細

- 既存の "walls" (value 1) を "Ground" に名前変更
- "Add value" ボタンで value 2, 3 を追加
- 各値の色をクリックして推奨色に設定

---

## 2. エンティティの定義

### 手順

1. 左パネルで **「Entities」** タブを開く
2. **「+ Add entity」** ボタンをクリック

### 必要なエンティティ

#### エンティティ1: Goal（ゴール）

- **Identifier**: `Goal`（大文字小文字を正確に）
- **Size**: 32 x 32 px
- **Color**: `#FFD700`（金色）
- **Pivot**: 0.0, 0.0（デフォルト）
- **説明**: レベルのゴール地点。プレイヤーが触れるとステージクリア

#### エンティティ2: Checkpoint（チェックポイント）

- **Identifier**: `Checkpoint`（大文字小文字を正確に）
- **Size**: 32 x 32 px
- **Color**: `#00FF00`（緑色）
- **Pivot**: 0.0, 0.0（デフォルト）
- **説明**: チェックポイント。プレイヤーが触れると起動し、死亡時のリスポーン地点になる

### 設定のポイント

⚠️ **重要**: Identifier名は大文字小文字を含めて正確に `Goal` と `Checkpoint` にすること（コード内で使用）

---

## 3. Entityレイヤーの追加

### 手順

1. 左パネルで **「Layers」** タブに戻る
2. **「+ Add layer」** ボタンをクリック
3. レイヤータイプとして **「Entities」** を選択
4. 以下を設定：
   - **Identifier**: `Entities`
   - **Grid size**: 8 px（既存レイヤーと同じ）
   - **Display above IntGrid**: チェック（エンティティを上に表示）

---

## 4. レベルの編集

### IntGridレイヤーで地形を描画

1. 左パネルで **「IntGrid_layer」** を選択
2. 右側のツールパレットで描画したい値を選択（1=Ground, 2=Wall, 3=OneWayPlatform）
3. レベルビュー内をクリック/ドラッグして地形を描画
4. 基本的なプラットフォームと壁を配置

### Entitiesレイヤーでオブジェクトを配置

1. 左パネルで **「Entities」** レイヤーを選択
2. 右側のエンティティパレットから配置したいエンティティを選択
3. レベルビュー内をクリックして配置：
   - **Goal**: レベルの終了地点に1つ配置
   - **Checkpoint**: 途中のチェックポイント位置に配置（複数可）

### 推奨レイアウト

```
[プレイヤースタート] → [地面＋壁] → [Checkpoint] → [地面＋壁] → [Goal]
```

---

## 5. 保存とテスト

### 保存

1. `Ctrl+S` または `File > Save` で保存
2. `assets/levels/test.ldtk` が更新されることを確認

### テスト実行

```bash
# ゲームをビルドして実行
cargo run

# または開発モード（高速ビルド）
cargo run --features bevy-dev
```

### 確認項目

- [ ] LDtkレベルが画面に表示される
- [ ] IntGridタイル（地面・壁）が表示される
- [ ] Goalエンティティが金色の四角で表示される
- [ ] Checkpointエンティティが緑色の四角で表示される
- [ ] プレイヤーが地面に立てる
- [ ] プレイヤーが壁を通過できない
- [ ] Goalに触れるとステート遷移のログが表示される
- [ ] Checkpointに触れると起動ログが表示される

---

## トラブルシューティング

### エンティティが表示されない

**原因**: Identifier名が間違っている可能性
**解決**: LDtkエディタで `Goal` と `Checkpoint` のスペルを確認（大文字小文字も正確に）

### 地面に立てない

**原因**: IntGrid value 1 が "Ground" になっていない
**解決**: IntGrid値の設定を確認

### LDtkファイルが読み込まれない

**原因**: ファイルパスが間違っている
**解決**: `assets/config/levels.ron` で `ldtk_path: "levels/test.ldtk"` が正しいか確認

### コンソールエラー確認

```bash
# ゲーム実行時のログを確認
cargo run 2>&1 | grep -E "(ERROR|WARN|Loading LDtk)"
```

---

## 参考リソース

### LDtkエディタ学習

- [LDtk公式ドキュメント](https://ldtk.io/docs/)
- [Getting Started Guide](https://ldtk.io/docs/general/install/)
- [Interface Overview](https://ldtk.io/docs/general/editor-components/)
- [LDtk Hands-On Tutorial (動画)](https://gamefromscratch.com/ldtk-2d-world-builder-hands-on-tutorial/)

### コード内の対応

IntGrid値の登録: `crates/level/src/plugin.rs`
```rust
app.register_ldtk_int_cell::<SolidGroundBundle>(1);  // Ground
app.register_ldtk_int_cell::<SolidWallBundle>(2);    // Wall
app.register_ldtk_int_cell::<OneWayPlatformBundle>(3); // OneWayPlatform
```

エンティティの登録: `crates/level/src/plugin.rs`
```rust
app.register_ldtk_entity::<GoalBundle>("Goal");
app.register_ldtk_entity::<CheckpointBundle>("Checkpoint");
```

---

## 完了チェックリスト

- [ ] LDtkエディタで `test.ldtk` を開いた
- [ ] IntGrid値を設定（1=Ground, 2=Wall, 3=OneWayPlatform）
- [ ] Goalエンティティを定義
- [ ] Checkpointエンティティを定義
- [ ] Entitiesレイヤーを追加
- [ ] IntGridレイヤーで地形を描画
- [ ] Entitiesレイヤーでゴールとチェックポイントを配置
- [ ] ファイルを保存
- [ ] `cargo run` でゲーム実行して動作確認
- [ ] すべての確認項目をクリア

---

**最終更新**: 2026-01-24
