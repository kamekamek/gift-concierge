use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftInput {
    pub gift_type: String,
    pub price_range: String,
    pub relationship: String,
    pub event_type: String,
    pub additional_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftRecommendation {
    pub name: String,
    pub price: String,
    pub store: String,
    pub reason: String,
    pub etiquette_advice: String,
}

pub struct RecommendationService {
    perplexity_client: Client,
    api_key: String,
}

impl RecommendationService {
    pub fn new(api_key: String) -> Self {
        Self {
            perplexity_client: Client::new(),
            api_key,
        }
    }

    pub async fn generate_recommendations(
        &self,
        input: GiftInput,
    ) -> Result<Vec<GiftRecommendation>, Box<dyn Error>> {
        // OpenAIでプロンプト生成
        let prompt = self.generate_search_prompt(&input);
        
        // Perplexity APIで検索
        let search_results = self.search_gifts(&prompt).await?;
        
        // 結果を構造化
        let recommendations = self.process_search_results(search_results)?;
        
        Ok(recommendations)
    }

    fn generate_search_prompt(&self, input: &GiftInput) -> String {
        format!(
            "お返しギフトの提案: 受け取ったギフト: {}, 価格帯: {}, 関係: {}, イベント: {}, 備考: {}",
            input.gift_type,
            input.price_range,
            input.relationship,
            input.event_type,
            input.additional_notes.as_deref().unwrap_or("")
        )
    }

    async fn search_gifts(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // Perplexity APIの呼び出し実装
        let response = self.perplexity_client
            .post("https://api.perplexity.ai/search")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "query": prompt
            }))
            .send()
            .await?;

        let result = response.text().await?;
        Ok(result)
    }

    fn process_search_results(&self, _results: String) -> Result<Vec<GiftRecommendation>, Box<dyn Error>> {
        // 検索結果を構造化されたレコメンデーションに変換
        let recommendations = vec![
            GiftRecommendation {
                name: "商品名".to_string(),
                price: "価格".to_string(),
                store: "店舗".to_string(),
                reason: "選定理由".to_string(),
                etiquette_advice: "マナーアドバイス".to_string(),
            }
        ];
        
        Ok(recommendations)
    }
} 