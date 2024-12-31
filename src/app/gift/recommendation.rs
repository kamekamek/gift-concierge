use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    gift_database: HashMap<String, Vec<GiftRecommendation>>,
}

impl RecommendationEngine {
    pub fn new() -> Self {
        let mut gift_database = HashMap::new();
        
        // 上司・先輩向けギフト
        gift_database.insert("上司".to_string(), vec![
            GiftRecommendation {
                name: "高級ボールペン".to_string(),
                description: "ビジネスシーンで活躍する、上質な書き味のボールペン".to_string(),
                price: 15000,
                category: "文具".to_string(),
                url: None,
            },
            GiftRecommendation {
                name: "名入れ革小物".to_string(),
                description: "上質な革を使用した、名入れ可能なカードケース".to_string(),
                price: 20000,
                category: "革小物".to_string(),
                url: None,
            },
        ]);
        
        // 友人向けギフト
        gift_database.insert("友人".to_string(), vec![
            GiftRecommendation {
                name: "ワイヤレスイヤホン".to_string(),
                description: "高音質で使いやすい、最新のワイヤレスイヤホン".to_string(),
                price: 15000,
                category: "電化製品".to_string(),
                url: None,
            },
            GiftRecommendation {
                name: "グルメギフトセット".to_string(),
                description: "選りすぐりの食材を集めた、贅沢なグルメセット".to_string(),
                price: 10000,
                category: "食品".to_string(),
                url: None,
            },
        ]);
        
        // 親戚向けギフト
        gift_database.insert("親戚".to_string(), vec![
            GiftRecommendation {
                name: "高級タオルセット".to_string(),
                description: "上質な肌触りの、ギフトボックス入りタオルセット".to_string(),
                price: 8000,
                category: "日用品".to_string(),
                url: None,
            },
            GiftRecommendation {
                name: "お茶・茶器セット".to_string(),
                description: "厳選された日本茶と、趣のある茶器のセット".to_string(),
                price: 12000,
                category: "食品".to_string(),
                url: None,
            },
        ]);

        Self {
            client: Client::new(),
            gift_database,
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
        let mut recommendations = if let Some(gifts) = self.gift_database.get(relationship) {
            gifts.clone()
        } else {
            // 関係性に基づくギフトが見つからない場合は、汎用的なギフトを提案
            vec![
                GiftRecommendation {
                    name: "ギフトカタログ".to_string(),
                    description: "受け取る方が好みの商品を選べる、カタログギフト".to_string(),
                    price: budget,
                    category: "カタログ".to_string(),
                    url: None,
                },
                GiftRecommendation {
                    name: "商品券".to_string(),
                    description: "様々な商品に使える、汎用性の高い商品券".to_string(),
                    price: budget,
                    category: "商品券".to_string(),
                    url: None,
                },
            ]
        };

        // 予算でフィルタリング
        recommendations.retain(|gift| gift.price <= budget);

        // 性別に基づくフィルタリング
        if let Some(gender) = gender {
            match gender {
                "male" => {
                    recommendations.retain(|gift| 
                        !gift.category.contains("化粧品") && 
                        !gift.category.contains("アクセサリー")
                    );
                }
                "female" => {
                    // 女性向けカテゴリーの優先順位を上げる
                    recommendations.sort_by(|a, b| {
                        let a_score = if a.category.contains("アクセサリー") || 
                                      a.category.contains("化粧品") { 0 } else { 1 };
                        let b_score = if b.category.contains("アクセサリー") || 
                                      b.category.contains("化粧品") { 0 } else { 1 };
                        a_score.cmp(&b_score)
                    });
                }
                _ => {}
            }
        }

        // まとめ買いの場合、単価を調整
        if is_bulk_gift {
            recommendations.iter_mut().for_each(|gift| {
                gift.price = (gift.price as f64 * 0.9) as i32; // 10%割引
                gift.description = format!("{}（まとめ買い割引適用）", gift.description);
            });
        }

        // 年齢に基づく並び替え
        if let Some(age) = age {
            let age_num = age.trim_end_matches("代").parse::<i32>().unwrap_or(30);
            recommendations.sort_by(|a, b| {
                let a_score = Self::calculate_age_relevance(&a.category, age_num);
                let b_score = Self::calculate_age_relevance(&b.category, age_num);
                b_score.cmp(&a_score)
            });
        }

        Ok(recommendations)
    }

    fn calculate_age_relevance(category: &str, age: i32) -> i32 {
        match category {
            c if c.contains("電化製品") => {
                if age < 40 { 3 } else { 1 }
            }
            c if c.contains("文具") => {
                if age > 30 { 3 } else { 2 }
            }
            c if c.contains("食品") => {
                if age > 50 { 3 } else { 2 }
            }
            _ => 2,
        }
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
            .await
            .unwrap();
        
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().all(|r| r.price <= 30000));
    }
} 