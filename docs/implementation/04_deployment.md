# デプロイメントガイド

## 1. デプロイメント環境

### 1.1 環境構成
- 開発環境 (Development)
- ステージング環境 (Staging)
- 本番環境 (Production)

### 1.2 必要なインフラストラクチャ
- Kubernetesクラスター
- PostgreSQLデータベース
- 監視システム
- バックアップシステム

## 2. コンテナ化

### 2.1 Dockerfile (`src/docker/Dockerfile`)
```dockerfile
# ビルドステージ
FROM rust:1.70 as builder
WORKDIR /usr/src/conselju
COPY . .
RUN cargo build --release

# 実行ステージ
FROM debian:bullseye-slim
COPY --from=builder /usr/src/conselju/target/release/conselju /usr/local/bin/
CMD ["conselju"]
```

### 2.2 Docker Compose (`src/docker/docker-compose.yaml`)
```yaml
version: '3.8'
services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/conselju
      - PERPLEXITY_API_KEY=${PERPLEXITY_API_KEY}
    depends_on:
      - db

  db:
    image: postgres:14
    environment:
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=conselju
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## 3. Kubernetesデプロイメント

### 3.1 デプロイメント設定 (`src/kubernetes/deployment.yaml`)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: conselju
spec:
  replicas: 3
  selector:
    matchLabels:
      app: conselju
  template:
    metadata:
      labels:
        app: conselju
    spec:
      containers:
      - name: conselju
        image: conselju:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: conselju-secrets
              key: database-url
        - name: PERPLEXITY_API_KEY
          valueFrom:
            secretKeyRef:
              name: conselju-secrets
              key: perplexity-api-key
```

### 3.2 サービス設定 (`src/kubernetes/service.yaml`)
```yaml
apiVersion: v1
kind: Service
metadata:
  name: conselju-service
spec:
  selector:
    app: conselju
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## 4. デプロイメントプロセス

### 4.1 開発環境へのデプロイ
```bash
# 開発環境の構築
docker-compose up -d

# データベースのマイグレーション
cargo run --bin migrate

# アプリケーションの起動確認
curl http://localhost:8080/health
```

### 4.2 ステージング/本番環境へのデプロイ
```bash
# コンテナイメージのビルド
docker build -t conselju:latest .

# Kubernetesへのデプロイ
kubectl apply -f kubernetes/
kubectl apply -f kubernetes/secrets.yaml
kubectl apply -f kubernetes/deployment.yaml
kubectl apply -f kubernetes/service.yaml

# デプロイメントの確認
kubectl get pods
kubectl get services
```

## 5. 監視とロギング

### 5.1 アプリケーションログ
```rust
// ログ設定
log4rs:
  appenders:
    console:
      kind: console
      encoder:
        pattern: "{d} - {l} - {m}{n}"
    file:
      kind: file
      path: "log/conselju.log"
      encoder:
        pattern: "{d} - {l} - {m}{n}"
```

### 5.2 メトリクス収集
- Prometheusによるメトリクス収集
- Grafanaによる可視化
- アラート設定

## 6. バックアップと復旧

### 6.1 データベースバックアップ
```bash
# 定期バックアップの設定
kubectl create cronjob postgres-backup --schedule="0 1 * * *" \
  --image=postgres:14 -- pg_dump -U postgres conselju

# バックアップの復元
psql -U postgres conselju < backup.sql
```

### 6.2 障害復旧計画
1. システム障害の検知
2. 影響範囲の特定
3. バックアップからの復旧
4. サービスの再開
5. 原因分析と再発防止

## 7. スケーリング戦略

### 7.1 水平スケーリング
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: conselju-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: conselju
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 80
```

### 7.2 データベーススケーリング
- レプリケーションの設定
- コネクションプールの最適化
- シャーディング戦略

## 8. セキュリティ対策

### 8.1 アプリケーションセキュリティ
- HTTPS/TLS設定
- CORS設定
- レート制限
- 入力バリデーション

### 8.2 インフラストラクチャセキュリティ
- ネットワークポリシー
- セキュリティグループ設定
- 証明書管理
- 脆弱性スキャン

## 9. 運用保守

### 9.1 定期メンテナンス
- セキュリティアップデート
- パフォーマンス最適化
- ログローテーション
- バックアップ検証

### 9.2 インシデント対応
1. 問題の検知と報告
2. 初期対応と影響軽減
3. 原因究明
4. 恒久対策の実施
5. 振り返りと改善

## 10. パフォーマンスチューニング

### 10.1 アプリケーションチューニング
- キャッシュ戦略の最適化
- データベースクエリの最適化
- 非同期処理の活用
- メモリ使用量の最適化

### 10.2 インフラストラクチャチューニング
- リソース割り当ての最適化
- ネットワーク設定の調整
- ロードバランサーの設定
- ストレージパフォーマンスの最適化
