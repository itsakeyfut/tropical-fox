このスプライトシートの構造

複数のアニメーション（idle, run, jump, attack等）が1枚にまとめられています。各行が異なるアニメーションを表現しています。

Bevyでの読み込み方法

// 各アニメーションの定義
struct FoxyAnimations {
    idle: AnimationIndices,
    run: AnimationIndices,
    jump: AnimationIndices,
    // ...
}

struct AnimationIndices {
    first: usize,
    last: usize,
}

// スプライトシート読み込み
let texture = asset_server.load("graphics/characters/players/Foxy/Spritesheet.png");
let layout = TextureAtlasLayout::from_grid(
    UVec2::new(32, 32),  // 各フレームのサイズ
    6,  // 列数
    10, // 行数
    None,
    None,
);

// アニメーション定義
let animations = FoxyAnimations {
    idle: AnimationIndices { first: 0, last: 3 },    // 1行目: 0-3
    run: AnimationIndices { first: 6, last: 11 },    // 2行目: 6-11
    jump: AnimationIndices { first: 12, last: 15 },  // 3行目: 12-15
    // ...
};

メリット・デメリット

メリット（現場で使われる理由）:
1. ✅ ファイル数削減 - 1ファイルで全アニメーション管理
2. ✅ 描画効率 - テクスチャ切り替え不要（バッチ描画可能）
3. ✅ メモリ効率 - GPU に1回ロードするだけ
4. ✅ ロード時間短縮 - ディスクI/O が1回

デメリット:
1. ❌ 管理が複雑 - どの行がどのアニメーションか把握が必要
2. ❌ 柔軟性低い - 1つのアニメーションだけ変更しづらい
3. ❌ コード側で定義 - インデックスを手動で管理

現場での使い分け

| 規模         | 方式            | 例                           |
|------------|---------------|-----------------------------|
| 小規模/プロトタイプ | アニメーション別ファイル  | foxy_idle.png, foxy_run.png |
| 中規模/インディー  | 統合スプライトシート    | Foxyのような形式                  |
| 大規模/商用     | テクスチャアトラス自動生成 | TexturePacker, Aseprite出力   |

.aseファイルについて

FoxyフォルダにFoxy.ase（Asepriteファイル）があります。これがソースファイルで、Spritesheet.pngはそこから書き出されたものです。

推奨アプローチ

この形式は現場で標準的なので、Bevyで使うことに問題ありません。ただし、コードでアニメーション範囲を定義する必要があります。

より管理しやすくするなら、.ronや**.json**でアニメーション定義を外部化するのも一般的です：

// foxy_animations.ron
(
    idle: (first: 0, last: 3),
    run: (first: 6, last: 11),
    jump: (first: 12, last: 15),
)