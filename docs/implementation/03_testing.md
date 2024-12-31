# テスト実装ガイド

## 1. テスト戦略

### 1.1 テストレベル
- ユニットテスト: 個々のコンポーネントの機能テスト
- 統合テスト: コンポーネント間の連携テスト
- エンドツーエンドテスト: システム全体の動作テスト
- パフォーマンステスト: システムの性能評価

### 1.2 テストカバレッジ目標
- ユニットテスト: 90%以上
- 統合テスト: 80%以上
- エンドツーエンドテスト: 主要フロー100%

## 2. ユニットテスト

### 2.1 メインアプリケーション (`src/test/main_test.rs`)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let app = ConseljuApp::new();
        assert!(app.is_initialized());
    }

    #[tokio::test]
    async fn test_api_routing() {
        let app = ConseljuApp::new();
        let response = app.handle_request("/api/v1/chat").await;
        assert!(response.is_ok());
    }
}
```

### 2.2 チャットボット (`src/test/chat_test.rs`)
```rust
#[cfg(test)]
mod chatbot_tests {
    #[test]
    fn test_conversation_flow() {
        let mut chatbot = Chatbot::new();
        let response = chatbot.process_message("こんにちは").await;
        assert_eq!(response.intent, Intent::Greeting);
    }

    #[test]
    fn test_user_info_collection() {
        let mut handler = ConversationHandler::new();
        handler.process_answer("上司からの就職祝い").await;
        assert_eq!(handler.collected_info.relationship, Relationship::Boss);
    }
}
```

### 2.3 ギフト推薦 (`src/test/gift_test.rs`)
```rust
#[cfg(test)]
mod recommendation_tests {
    #[tokio::test]
    async fn test_gift_search() {
        let recommender = GiftRecommendation::new();
        let criteria = SearchCriteria {
            relationship: Relationship::Boss,
            budget: 30000,
        };
        let results = recommender.recommend_gifts(criteria).await;
        assert_eq!(results.len(), 3);
    }
}
```

## 3. 統合テスト

### 3.1 コンポーネント間の統合
```rust
#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_chat_to_recommendation_flow() {
        let mut app = ConseljuApp::new();
        let chat_response = app.process_chat_message("上司から5万円の祝い金をもらいました").await;
        let recommendations = app.get_recommendations().await;
        
        assert!(recommendations.len() > 0);
        assert!(recommendations[0].price <= 50000);
    }
}
```

### 3.2 データベース統合
```rust
#[cfg(test)]
mod database_integration_tests {
    #[tokio::test]
    async fn test_user_data_persistence() {
        let mut app = ConseljuApp::new();
        let user_info = UserInfo {
            relationship: Relationship::Boss,
            budget: 30000,
        };
        
        app.save_user_info(user_info).await?;
        let loaded_info = app.load_user_info().await?;
        assert_eq!(user_info, loaded_info);
    }
}
```

## 4. エンドツーエンドテスト

### 4.1 主要フロー
```rust
#[cfg(test)]
mod e2e_tests {
    #[tokio::test]
    async fn test_complete_gift_recommendation_flow() {
        let mut app = ConseljuApp::new();
        
        // 会話開始
        let greeting = app.start_conversation().await?;
        assert!(greeting.contains("こんにちは"));
        
        // ユーザー情報入力
        let info_response = app.process_message("上司から3万円もらいました").await?;
        assert!(info_response.contains("予算"));
        
        // 推薦取得
        let recommendations = app.get_final_recommendations().await?;
        assert_eq!(recommendations.len(), 3);
    }
}
```

### 4.2 エラーケース
```rust
#[cfg(test)]
mod error_case_tests {
    #[tokio::test]
    async fn test_invalid_input_handling() {
        let mut app = ConseljuApp::new();
        let response = app.process_message("不正な入力").await;
        assert!(response.is_error());
        assert!(response.error_message.contains("理解できません"));
    }
}
```

## 5. パフォーマンステスト

### 5.1 負荷テスト
```rust
#[cfg(test)]
mod performance_tests {
    #[tokio::test]
    async fn test_concurrent_requests() {
        let app = Arc::new(ConseljuApp::new());
        let mut handles = vec![];
        
        for _ in 0..100 {
            let app_clone = app.clone();
            handles.push(tokio::spawn(async move {
                app_clone.process_message("テストメッセージ").await
            }));
        }
        
        let results = join_all(handles).await;
        assert!(results.iter().all(|r| r.is_ok()));
    }
}
```

### 5.2 レスポンス時間測定
```rust
#[cfg(test)]
mod response_time_tests {
    #[tokio::test]
    async fn test_recommendation_response_time() {
        let app = ConseljuApp::new();
        let start = Instant::now();
        let _ = app.get_recommendations().await?;
        assert!(start.elapsed() < Duration::from_secs(2));
    }
}
```

## 6. テスト実行手順

### 6.1 ローカルテスト実行
```bash
# 全テストの実行
cargo test

# 特定のテストの実行
cargo test chat_test
cargo test integration

# テストカバレッジの確認
cargo tarpaulin
```

### 6.2 CI/CDパイプラインでのテスト
```yaml
test:
  script:
    - cargo test
    - cargo tarpaulin --out Xml
    - cargo clippy
```

## 7. テスト保守

### 7.1 テストコードのベストプラクティス
- テストケースの独立性を保つ
- テストデータの適切な管理
- モックの効果的な使用
- テストの可読性の維持

### 7.2 テスト更新手順
1. 新機能追加時の対応するテストケース作成
2. バグ修正時の回帰テスト追加
3. 定期的なテストケースの見直しと更新
4. テストパフォーマンスの最適化
