use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftRequest {
    received_gift: String,
    price_range: PriceRange,
    relationship: Relationship,
    event_type: EventType,
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceRange {
    min: u32,
    max: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Relationship {
    Boss,
    Colleague,
    Friend,
    Family,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    Wedding,
    Birth,
    Celebration,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftRecommendation {
    name: String,
    price: u32,
    store: String,
    reason: String,
    manner_advice: String,
}

#[derive(Debug, Serialize)]
struct PerplexityRequest {
    query: String,
}

#[derive(Debug, Deserialize)]
struct PerplexityResponse {
    answer: String,
}

pub struct GiftRecommender {
    client: Client,
    api_key: String,
}

impl GiftRecommender {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get_recommendations(&self, request: GiftRequest) -> Result<Vec<GiftRecommendation>> {
        let query = self.build_search_query(&request);
        
        let perplexity_request = PerplexityRequest { query };
        
        let response = self.client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&perplexity_request)
            .send()
            .await?;

        let perplexity_response: PerplexityResponse = response.json().await?;
        
        // レスポンスをパースしてギフト推薦に変換
        self.parse_recommendations(&perplexity_response.answer)
    }

    fn build_search_query(&self, request: &GiftRequest) -> String {
        format!(
            "以下の条件に合うお返しのギフトを3つ提案してください。各提案には商品名、価格、購入店舗、選定理由、マナーアドバイスを含めてください：
            - 受け取ったギフト: {}
            - 予算: {}円-{}円
            - 関係: {}
            - イベント: {}
            {}",
            request.received_gift,
            request.price_range.min,
            request.price_range.max,
            self.relationship_to_string(&request.relationship),
            self.event_type_to_string(&request.event_type),
            request.notes.as_deref().unwrap_or("")
        )
    }

    fn parse_recommendations(&self, response: &str) -> Result<Vec<GiftRecommendation>> {
        // TODO: 実際のレスポンスパースロジックを実装
        // 現在はダミーデータを返す
        Ok(vec![
            GiftRecommendation {
                name: "高級タオルセット".to_string(),
                price: 5000,
                store: "高島屋".to_string(),
                reason: "実用的で上質な贈り物として適切です".to_string(),
                manner_advice: "包装は二重包みで、のしは「御返し」を使用します".to_string(),
            }
        ])
    }

    fn relationship_to_string(&self, relationship: &Relationship) -> &str {
        match relationship {
            Relationship::Boss => "上司",
            Relationship::Colleague => "同僚",
            Relationship::Friend => "友人",
            Relationship::Family => "家族",
            Relationship::Other => "その他",
        }
    }

    fn event_type_to_string(&self, event_type: &EventType) -> &str {
        match event_type {
            EventType::Wedding => "結婚祝い",
            EventType::Birth => "出産祝い",
            EventType::Celebration => "お祝い",
            EventType::Other => "その他",
        }
    }
} 