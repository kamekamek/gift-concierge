use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gift {
    pub name: String,
    pub price: i32,
    pub description: String,
    pub image_url: Option<String>,
    pub category: String,
    pub rating: f32,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct GiftRecommender {
    cache: Arc<Mutex<GiftCache>>,
}

#[derive(Debug, Default)]
struct GiftCache {
    gifts: Vec<Gift>,
    last_query: Option<SearchQuery>,
}

#[derive(Debug, PartialEq, Eq)]
struct SearchQuery {
    budget: i32,
    occasion: String,
    age: Option<i32>,
    gender: Option<String>,
}

impl GiftRecommender {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(GiftCache::default())),
        }
    }

    pub async fn get_recommendations(
        &self,
        budget: Option<i32>,
        occasion: Option<&str>,
        age: Option<i32>,
        gender: Option<&str>,
    ) -> Result<Vec<Gift>> {
        let budget = budget.unwrap_or(30000);
        let occasion = occasion.unwrap_or("就職祝い").to_string();

        let query = SearchQuery {
            budget,
            occasion: occasion.clone(),
            age,
            gender: gender.map(String::from),
        };

        let mut cache = self.cache.lock().await;
        
        // キャッシュチェック
        if let Some(last_query) = &cache.last_query {
            if last_query == &query && !cache.gifts.is_empty() {
                return Ok(cache.gifts.clone());
            }
        }

        // Perplexity APIを使用してギフトを検索
        let gifts = self.search_gifts(&query).await?;
        
        // 結果をキャッシュに保存
        cache.gifts = gifts.clone();
        cache.last_query = Some(query);

        Ok(gifts)
    }

    async fn search_gifts(&self, query: &SearchQuery) -> Result<Vec<Gift>> {
        // TODO: Perplexity APIを使用した実際の検索を実装
        // 現在はモックデータを返す
        Ok(vec![
            Gift {
                name: "高級万年筆セット".to_string(),
                price: query.budget - 5000,
                description: "ビジネスシーンで活躍する高級万年筆とケースのセット".to_string(),
                image_url: Some("https://example.com/pen.jpg".to_string()),
                category: "文具".to_string(),
                rating: 4.5,
                source: "高級文具専門店".to_string(),
            },
            Gift {
                name: "革製ビジネスバッグ".to_string(),
                price: query.budget,
                description: "上質な革を使用したビジネスバッグ。収納力も抜群".to_string(),
                image_url: Some("https://example.com/bag.jpg".to_string()),
                category: "バッグ".to_string(),
                rating: 4.8,
                source: "有名バッグブランド".to_string(),
            },
            Gift {
                name: "高級腕時計".to_string(),
                price: query.budget + 5000,
                description: "ビジネスシーンにふさわしい高級腕時計".to_string(),
                image_url: Some("https://example.com/watch.jpg".to_string()),
                category: "アクセサリー".to_string(),
                rating: 4.7,
                source: "時計専門店".to_string(),
            },
        ])
    }
} 