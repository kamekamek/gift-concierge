# 開発環境セットアップ手順

## 前提条件

以下のツールが必要です：
- Rust (最新の安定版)
- Docker
- Docker Compose
- Kubernetes (オプション: 本番環境用)
- PostgreSQL

## 1. Rustのインストール

```bash
# Rustupのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Rustのバージョン確認
rustc --version
cargo --version
```

## 2. プロジェクトのセットアップ

```bash
# プロジェクトのクローン
git clone [repository-url]
cd conselju

# 依存関係のインストール
cargo build
```

## 3. 環境変数の設定

1. 環境変数ファイルの準備
```bash
# 開発環境用の.envファイルをコピー
cp src/config/.env.development src/config/.env
```

2. 必要な環境変数の設定
```env
# Perplexity API設定
PERPLEXITY_API_KEY=your_api_key

# データベース設定
DATABASE_URL=postgresql://username:password@localhost:5432/conselju_db

# アプリケーション設定
APP_ENV=development
APP_PORT=8080
```

## 4. データベースのセットアップ

### ローカル開発環境

```bash
# PostgreSQLコンテナの起動
docker-compose up -d postgres

# データベースの初期化
cargo run --bin db_init
```

### Docker環境

```bash
# 開発環境の構築
docker-compose up -d
```

## 5. アプリケーションの起動

### ローカル開発モード

```bash
# 開発サーバーの起動
cargo run
```

### Docker環境

```bash
# コンテナのビルドと起動
docker-compose up --build
```

## 6. 動作確認

1. ヘルスチェック
```bash
curl http://localhost:8080/health
```

2. APIテスト
```bash
curl http://localhost:8080/api/v1/chat
```

## トラブルシューティング

### よくある問題と解決方法

1. データベース接続エラー
```bash
# PostgreSQLサービスの状態確認
docker-compose ps
docker-compose logs postgres
```

2. 依存関係のエラー
```bash
# Cargoのキャッシュクリア
cargo clean
cargo build
```

3. 環境変数の問題
```bash
# 環境変数の確認
printenv | grep PERPLEXITY
printenv | grep DATABASE
```

## 開発ツール

### 推奨VSCode拡張機能

- rust-analyzer: Rust言語サポート
- Better TOML: TOML設定ファイル編集
- Docker: Dockerファイル管理
- GitLens: Git操作の拡張

### デバッグ設定

`launch.json`の設定例：
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable",
            "cargo": {
                "args": ["build"],
                "filter": {
                    "name": "conselju",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
