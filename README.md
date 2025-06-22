# 関数合成スタイルのWarp APIサーバー

このプロジェクトは、Rustの`warp`フレームワークとMySQLを用い、純粋な関数合成スタイル・クリーンアーキテクチャで設計されたAPIサーバーの概念実証（PoC）です。

## 特徴

- 純粋関数型プログラミングスタイル
- 関数合成パターンの徹底適用
- クリーンアーキテクチャによる明確なレイヤ分離
- `warp`のフィルター合成による宣言的APIルーティング
- MySQL対応（SQLite→MySQLで高並列・大量データ投入も安定動作）
- r2d2によるDBコネクションプール
- 高負荷（10万件/100並列）投入・メモリ監視済み

## プロジェクト構造

```
.
├── crates/
│   ├── domain/         # ドメインモデル・バリューオブジェクト・例外
│   ├── application/    # ユースケース・サービス・リポジトリトレイト
│   ├── infrastructure/ # DB実装（MySQL/SQLite）・外部サービス
│   ├── presentation/   # APIエンドポイント・Warpハンドラー
│   └── cores/          # 関数合成ユーティリティ
├── src/
│   └── main.rs         # アプリケーションのエントリーポイント
├── Dockerfile          # APIサーバ用Dockerfile
├── docker-compose.yml  # MySQL＋APIサーバ一括起動
└── ...
```

## セットアップ・実行方法

### 1. Docker ComposeでMySQL＋APIサーバを起動

```bash
docker compose up --build
```

- サーバは http://localhost:3030 で起動します
- MySQLは`user:password@db:3306/clodure_db`で自動起動
- `.env`や`DATABASE_URL`はdocker-compose.ymlにより自動設定

### 2. ローカルで直接実行（MySQLが別途必要）

```bash
export DATABASE_URL="mysql://user:password@localhost:3306/clodure_db"
cargo run
```

## API エンドポイント

- `GET    /users`         : すべてのユーザーを取得
- `GET    /users/:id`     : IDによるユーザー取得
- `POST   /users`         : 新規ユーザー作成
- `PUT    /users/:id`     : ユーザー更新
- `DELETE /users/:id`     : ユーザー削除

## リクエスト例

### ユーザー作成

```bash
curl -X POST http://localhost:3030/users \
  -H "Content-Type: application/json" \
  -d '{"name":"田中太郎","email":"tanaka@example.com"}'
```

### ユーザー取得

```bash
curl http://localhost:3030/users/<ユーザーID>
```

## 高負荷・メモリ検証

- 10万件のユーザーを100並列で投入するスクリプト（`generate_random_users.sh`）を同梱
- メモリ監視スクリプト（`memory_monitoring_and_load_testing.sh`）でAPIサーバの安定性・メモリ使用量を自動記録
- MySQL構成で10分台/10万件・メモリ40MB台で安定動作を確認済み

## 設計思想・アーキテクチャ

- **関数合成**: warpのFilterやサービス層を関数合成で宣言的に構築
- **クリーンアーキテクチャ**: ドメイン→アプリケーション→インフラ→プレゼンテーションの依存逆転
- **副作用の制御**: DBや外部I/Oはインフラ層に集約し、上位層は純粋関数中心
- **リソース共有**: `Arc`によるスレッド安全な共有と不変性保証
- **テスト容易性**: リポジトリはトレイトで抽象化し、モック差し替え可能