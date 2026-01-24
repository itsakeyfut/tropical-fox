公式ドキュメント

**LDtk（Level Designer Toolkit）**は、Dead Cellsのディレクターが開発した無料のオープンソース2Dレベルエディタです。

主要な学習リソース

1. 公式ドキュメント


    - https://ldtk.io/docs/ - 公式ドキュメントハブ
    - https://ldtk.io/docs/general/install/ - インストールと初期設定
    - https://ldtk.io/docs/general/editor-components/ - エディタのUI説明

2. ダウンロード


    - https://ldtk.io/ - Windows/Mac/Linux版をダウンロード
    - https://deepnight.itch.io/ldtk - 代替ダウンロードページ

3. チュートリアル動画


    - https://gamefromscratch.com/ldtk-2d-world-builder-hands-on-tutorial/ - GameFromScratchの実践チュートリアル
    - https://gamefromscratch.com/ldtk-the-level-designer-toolkit/ - 概要と基本機能

4. コミュニティ


    - https://github.com/deepnight/ldtk - ソースコードとIssues
    - 公式Discordコミュニティ（ドキュメントサイトからリンクあり）

本プロジェクトで必要な基本操作

現在のTropical Foxプロジェクトでは、以下の機能を使用しています：

1. IntGrid Layer（衝突判定用）

- 値1: Ground（地面）
- 値2: Wall（壁）
- 値3: OneWayPlatform（一方通行プラットフォーム）

2. Entity Layer（オブジェクト配置用）

- "Goal" エンティティ - ゴール地点
- "Checkpoint" エンティティ - チェックポイント

日本語リソース

LDtkは比較的新しいツールなので、日本語の情報は限られていますが、公式ドキュメントは画像が多く直感的なので、英語でも理解しやすいです。

推奨学習ステップ

1. 公式サイトからダウンロード → https://ldtk.io/
2. Getting Startedを読む → 基本的な概念を理解
3. GameFromScratchのチュートリアル動画 → 実際の操作を見て学ぶ
4. サンプルプロジェクトで練習 → IntGridとEntityの配置を試す
5. 本プロジェクトで実践 → assets/levels/test.ldtkを編集

何か具体的な操作で困ったら、お気軽に質問してください！

Sources:

- https://ldtk.io/docs/
- https://ldtk.io/docs/general/install/
- https://ldtk.io/
- https://ldtk.io/docs/general/editor-components/
- https://gamefromscratch.com/ldtk-2d-world-builder-hands-on-tutorial/
- https://github.com/deepnight/ldtk
