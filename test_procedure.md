# お返しコンシェルジュBOT テスト手順書

## 1. 環境セットアップ

### 1.1 必要なツールのインストール
```bash
# Rustのインストール（未インストールの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQLのインストール（未インストールの場合）
brew install postgresql@13

# Docker Desktop のインストール（未インストールの場合）
brew install --cask docker
```

### 1.2 環境変数の設定
1. `.env.example`を`.env`にコピー
2. 以下の環境変数を設定：
   - `DB_USERNAME`と`DB_PASSWORD`を適切な値に変更
   - `PERPLEXITY_API_KEY`に有効なAPIキーを設定

### 1.3 データベースの準備
```bash
# PostgreSQLサービスの起動
brew services start postgresql@13

# データベースの作成
createdb gift_advisor_db
```

## 2. ユニットテストの実行

### 2.1 個別のコンポーネントテスト
```bash
# チャットボットのテスト
cargo test --package my-project --lib "app::chat::chatbot::tests"

# 意図分類器のテスト
cargo test --package my-project --lib "app::nlp::intent_classifier::tests"

# ギフト推薦エンジンのテスト
cargo test --package my-project --lib "app::gift::recommendation::tests"
```

### 2.2 全ユニットテストの実行
```bash
cargo test --lib
```

## 3. 統合テストの実行

### 3.1 統合テストの準備
```bash
# テスト用のデータベースを作成
createdb gift_advisor_test_db

# 環境変数の設定
export DATABASE_URL="postgres://gift_advisor:your_password@localhost:5432/gift_advisor_test_db"
```

### 3.2 統合テストの実行
```bash
# すべての統合テストを実行
cargo test --test integration_tests

# 特定のテストケースを実行
cargo test --test integration_tests test_complete_gift_recommendation_flow
cargo test --test integration_tests test_error_handling
cargo test --test integration_tests test_multilingual_support
```

## 4. 負荷テストの実行

### 4.1 負荷テストの準備
```bash
# システムリソースの制限を確認
ulimit -n 2048
```

### 4.2 負荷テストの実行
```bash
# 同時実行負荷テスト
cargo test --test load_tests test_concurrent_load

# 持続的負荷テスト
cargo test --test load_tests test_sustained_load

# キャッシュパフォーマンステスト
cargo test --test load_tests test_cache_performance
```

## 5. エンドツーエンドテスト

### 5.1 アプリケーションの起動
```bash
# 開発モードでアプリケーションを起動
cargo run
```

### 5.2 手動テストシナリオ
1. 基本的な会話フロー
   ```
   入力: こんにちは
   期待: 挨拶と案内メッセージ

   入力: 上司へのお返しを探しています
   期待: 予算に関する質問

   入力: 3万円くらいです
   期待: 人数に関する質問

   入力: 1人分です
   期待: 性別に関する質問

   入力: 男性です
   期待: 年齢に関する質問

   入力: 50代です
   期待: ギフトの推薦リスト
   ```

2. エラーケースの確認
   ```
   入力: （空文字列）
   期待: エラーメッセージ

   入力: 予算は未定です
   期待: 予算の具体的な指定を求めるメッセージ
   ```

3. 多言語対応の確認
   ```
   入力: hello
   期待: 英語での応答

   入力: help
   期待: 英語でのヘルプメッセージ
   ```

## 6. パフォーマンス検証

### 6.1 レスポンス時間の計測
```bash
# 応答時間の計測（curlを使用）
time curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{"user_id": "test_user", "message": "こんにちは"}'
```

### 6.2 メモリ使用量の監視
```bash
# メモリ使用量の確認
ps aux | grep my-project
```

## 7. セキュリティテスト

### 7.1 依存関係の脆弱性チェック
```bash
# cargo-audit のインストール
cargo install cargo-audit

# 脆弱性のチェック
cargo audit
```

### 7.2 環境変数の検証
1. 必須環境変数が設定されていない場合のエラーハンドリング確認
2. 無効なAPIキーでの動作確認

## 8. CI/CDパイプラインのテスト

### 8.1 ローカルでのGitHub Actions確認
```bash
# act のインストール（GitHub Actionsローカル実行ツール）
brew install act

# ワークフローのローカル実行
act -n  # dry-run
act    # 実行
```

### 8.2 デプロイメントテスト
```bash
# Dockerイメージのビルド
docker build -t gift-advisor .

# コンテナの起動
docker run -d -p 8080:8080 --env-file .env gift-advisor

# ヘルスチェック
curl http://localhost:8080/health
```

## 9. テスト結果の確認

### 9.1 テストカバレッジの計測
```bash
# tarpaulin のインストール
cargo install cargo-tarpaulin

# カバレッジレポートの生成
cargo tarpaulin --out Html
```

### 9.2 テストレポートの確認
1. `target/tarpaulin/coverage.html`を確認
2. 統合テストの出力ログを確認
3. 負荷テストの結果メトリクスを確認

## 10. トラブルシューティング

### 10.1 一般的な問題の解決
- データベース接続エラー
  ```bash
  # PostgreSQLの状態確認
  pg_isready
  ```

- 権限の問題
  ```bash
  # データベースユーザーの権限確認
  psql -U gift_advisor -d gift_advisor_db -c "\du"
  ```

### 10.2 ログの確認
```bash
# アプリケーションログの確認
tail -f logs/app.log

# テストログの確認
RUST_LOG=debug cargo test
``` 