# コアコンポーネントの実装手順

## 1. メインアプリケーション (`src/app/main.rs`)

### 1.1 基本構造
```rust
// メインアプリケーションの構造
pub struct ConseljuApp {
    chatbot: Chatbot,
    recommendation: GiftRecommendation,
    user_records: UserRecords,
}

// 初期化処理
impl ConseljuApp {
    pub fn new() -> Self {
        // 各コンポーネントの初期化
    }

    pub async fn run() -> Result<(), Error> {
        // アプリケーションのメインループ
    }
}
```

### 1.2 実装順序
1. アプリケーションの基本構造の実装
2. 依存コンポーネントの統合
3. APIルーティングの設定
4. エラーハンドリングの実装
5. ログ機能の追加

## 2. チャットボットモジュール

### 2.1 チャットボット本体 (`src/app/chat/chatbot.rs`)
```rust
pub struct Chatbot {
    conversation_handler: ConversationHandler,
    intent_classifier: IntentClassifier,
}

impl Chatbot {
    // 会話の開始
    pub async fn start_conversation(&mut self) -> Result<(), ChatError> {}
    
    // メッセージの処理
    pub async fn process_message(&mut self, message: String) -> Result<Response, ChatError> {}
}
```

### 2.2 会話ハンドラー (`src/app/chat/conversation_handler.rs`)
```rust
pub struct ConversationHandler {
    current_state: ConversationState,
    collected_info: UserInfo,
}

impl ConversationHandler {
    // 次の質問の生成
    pub fn generate_next_question(&self) -> Question {}
    
    // 回答の処理
    pub fn process_answer(&mut self, answer: String) -> Result<(), HandlerError> {}
}
```

### 2.3 実装順序
1. 基本的な会話フローの実装
2. 意図分類システムの統合
3. エラーハンドリングの追加
4. ユーザー情報収集ロジックの実装
5. マナー質問対応の追加

## 3. ギフト推薦システム (`src/app/gift/recommendation.rs`)

### 3.1 基本構造
```rust
pub struct GiftRecommendation {
    perplexity_client: PerplexityClient,
    gift_cache: GiftCache,
}

impl GiftRecommendation {
    // ギフトの検索と推薦
    pub async fn recommend_gifts(&self, criteria: SearchCriteria) -> Vec<Gift> {}
    
    // 検索結果の絞り込み
    fn filter_results(&self, results: Vec<Gift>) -> Vec<Gift> {}
}
```

### 3.2 実装順序
1. Perplexity APIクライアントの実装
2. 検索ロジックの実装
3. フィルタリングロジックの実装
4. キャッシュシステムの統合
5. レスポンス最適化

## 4. データベース管理

### 4.1 ユーザー記録 (`src/app/database/user_record.rs`)
```rust
pub struct UserRecords {
    db_connection: DbConnection,
}

impl UserRecords {
    // ユーザー情報の保存
    pub async fn save_user_info(&self, info: UserInfo) -> Result<(), DbError> {}
    
    // ギフト履歴の取得
    pub async fn get_gift_history(&self, user_id: UserId) -> Result<Vec<Gift>, DbError> {}
}
```

### 4.2 ギフトキャッシュ (`src/app/database/gift_cache.rs`)
```rust
pub struct GiftCache {
    cache: HashMap<SearchCriteria, Vec<Gift>>,
}

impl GiftCache {
    // キャッシュの取得と更新
    pub fn get_or_update(&mut self, criteria: SearchCriteria) -> Option<Vec<Gift>> {}
}
```

### 4.3 実装順序
1. データベーススキーマの設計
2. CRUD操作の実装
3. キャッシュシステムの実装
4. マイグレーションシステムの設定
5. バックアップ戦略の実装

## 5. 自然言語処理 (`src/app/nlp/intent_classifier.rs`)

### 5.1 基本構造
```rust
pub struct IntentClassifier {
    model: NlpModel,
}

impl IntentClassifier {
    // ユーザー入力の意図分類
    pub fn classify_intent(&self, input: &str) -> Intent {}
    
    // 関係性の抽出
    pub fn extract_relationship(&self, input: &str) -> Option<Relationship> {}
}
```

### 5.2 実装順序
1. 基本的な意図分類ロジックの実装
2. 関係性抽出機能の追加
3. 金額認識機能の実装
4. エッジケースの処理追加
5. パフォーマンス最適化

## 6. 統合とテスト

### 6.1 統合手順
1. 各コンポーネントの単体テスト
2. コンポーネント間の統合テスト
3. エンドツーエンドテスト
4. パフォーマンステスト
5. セキュリティテスト

### 6.2 品質保証
- ユニットテストの作成
- 統合テストの実装
- ドキュメントの生成
- コードレビュープロセスの確立
