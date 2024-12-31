use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftRecommendation {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub category: String,
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct RecommendationEngine {
    client: Client,
}

impl RecommendationEngine {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_recommendations(
        &self,
        relationship: &str,
        budget: i32,
        is_bulk_gift: bool,
        gender: Option<&str>,
        age: Option<&str>,
    ) -> Result<Vec<GiftRecommendation>> {
        // TODO: Perplexity APIとの連携を実装
        // 現在はモックデータを返す
        let mock_recommendations = match relationship {
            "上司" => vec![
                GiftRecommendation {
                    name: "高級茶葉セット".to_string(),
                    description: "厳選された日本茶のセット。上品な香りと味わいが特徴です。".to_string(),
                    price: budget / 2,
                    category: "食品".to_string(),
                    url: None,
                },
                GiftRecommendation {
                    name: "伝統工芸品".to_string(),
                    description: "熟練の職人による手作りの工芸品。末永く使っていただけます。".to_string(),
                    price: (budget as f32 * 0.7) as i32,
                    category: "工芸品".to_string(),
                    url: None,
                },
                GiftRecommendation {
                    name: "高級タオルセット".to_string(),
                    description: "今治タオルの最高級ライン。贈り物に相応しい上質な仕上がり。".to_string(),
                    price: budget / 3,
                    category: "日用品".to_string(),
                    url: None,
                },
            ],
            "友人" => vec![
                GiftRecommendation {
                    name: "グルメカタログギフト".to_string(),
                    description: "選べる食事券。好みに合わせて使えます。".to_string(),
                    price: budget / 2,
                    category: "カタログギフト".to_string(),
                    url: None,
                },
                GiftRecommendation {
                    name: "ペアグラスセット".to_string(),
                    description: "記念日に使える高級グラスのセット。".to_string(),
                    price: (budget as f32 * 0.6) as i32,
                    category: "食器".to_string(),
                    url: None,
                },
                GiftRecommendation {
                    name: "体験ギフト".to_string(),
                    description: "思い出に残る体験を贈ります。".to_string(),
                    price: budget,
                    category: "体験".to_string(),
                    url: None,
                },
            ],
            _ => vec![
                GiftRecommendation {
                    name: "カタログギフト".to_string(),
                    description: "幅広い商品から選べる汎用的なギフト。".to_string(),
                    price: budget,
                    category: "カタログギフト".to_string(),
                    url: None,
                },
            ],
        };

        Ok(mock_recommendations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_recommendations() {
        let engine = RecommendationEngine::new();
        let recommendations = engine
            .get_recommendations("上司", 30000, false, Some("male"), Some("50代"))
            .await;
        assert!(recommendations.is_ok());
        let recommendations = recommendations.unwrap();
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().all(|r| r.price <= 30000));
    }
} 