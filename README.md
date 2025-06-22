# 関数合成スタイルのWarp APIサーバー

このプロジェクトは、Rustの`warp`フレームワークを使用して、純粋な関数合成スタイルでAPIサーバーを実装したPoC（概念実証）です。

## 特徴

- 純粋な関数型プログラミングスタイル
- 関数合成パターンの適用
- クリーンアーキテクチャに基づくモジュール分割
- `warp`のフィルター合成による宣言的APIルーティング

## プロジェクト構造

```
.
├── crates/
│   ├── domain/        # ドメインモデルと例外
│   ├── application/   # ユースケースとサービス
│   ├── infrastructure/# データアクセスと外部サービス
│   ├── presentation/  # APIエンドポイントとハンドラー
│   └── cores/         # 関数合成ユーティリティ
└── src/
    └── main.rs        # アプリケーションのエントリーポイント
```

## 実行方法

```bash
cargo run
```

サーバーは http://127.0.0.1:3030 で起動します。

## API エンドポイント

- `GET /users` - すべてのユーザーを取得
- `GET /users/:id` - IDによるユーザー取得
- `POST /users` - 新規ユーザー作成
- `PUT /users/:id` - ユーザー更新
- `DELETE /users/:id` - ユーザー削除

## リクエスト例

### ユーザー作成

```bash
curl -X POST http://localhost:3030/users \
  -H "Content-Type: application/json" \
  -d '{"name":"田中太郎","email":"tanaka@example.com"}'
```

### ユーザー取得

```bash
curl http://localhost:3030/users/:id
```

## 設計思想

このプロジェクトは、関数合成を中心に設計されています。特に以下の点に注目してください：

1. `warp`のフィルターを使用した宣言的API定義
2. 副作用を制限し、純粋関数を中心に構成
3. `Arc`によるリソース共有と不変性の保証
4. コアモジュールにある関数合成ユーティリティ

各レイヤーは明確に分離され、依存関係は内側に向かって流れるようになっています。
