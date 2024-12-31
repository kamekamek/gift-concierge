use reqwest;
use serde::{Serialize, Deserialize};
use crate::app::error::{ChatError, Result};
use log::{info, debug, error};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftRecommendation {
    pub name: String,
    pub description: String,
    pub price: Option<u32>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub rating: Option<f32>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PerplexityResponse {
    results: Vec<SearchResult>,
    total: u32,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    title: String,
    snippet: String,
    url: Option<String>,
    image_url: Option<String>,
    price: Option<String>,
}

pub struct GiftRecommender {
    api_client: reqwest::Client,
    cache: crate::app::database::GiftCache,
}

impl GiftRecommender {
    pub fn new() -> Self {
        Self {
            api_client: reqwest::Client::new(),
            cache: crate::app::database::GiftCache::new(),
        }
    }

    pub async fn get_recommendations(
        &self,
        relationship: &str,
        min_amount: u32,
        max_amount: u32,
        preferences: &GiftPreferences,
    ) -> Result<Vec<GiftRecommendation>> {
        debug!("Searching gifts for {} (¥{}-¥{})", relationship, min_amount, max_amount);

        // キャッシュをチェック
        let cache_key = self.generate_cache_key(relationship, min_amount, max_amount, preferences);
        if let Some(cached_results) = self.cache.get(&cache_key) {
            info!("Found cached results for {}", cache_key);
            return Ok(cached_results);
        }

        // Perplexity APIを呼び出し
        let results = self.search_gifts(relationship, min_amount, max_amount, preferences).await?;
        
        // 結果をフィルタリング
        let filtered_results = self.filter_results(results, min_amount, max_amount, preferences);
        
        // キャッシュに保存
        self.cache.set(&cache_key, &filtered_results);
        
        Ok(filtered_results)
    }

    async fn search_gifts(
        &self,
        relationship: &str,
        min_amount: u32,
        max_amount: u32,
        preferences: &GiftPreferences,
    ) -> Result<Vec<SearchResult>> {
        let api_key = env::var("PERPLEXITY_API_KEY")
            .map_err(|_| ChatError::InternalError("Perplexity API key not found".to_string()))?;

        let query = self.build_search_query(relationship, min_amount, max_amount, preferences);
        let url = format!("https://api.perplexity.ai/search?q={}&key={}", query, api_key);

        let response = self.api_client.get(&url)
            .send()
            .await
            .map_err(|e| ChatError::ApiError(format!("Perplexity API request failed: {}", e)))?;

        let data: PerplexityResponse = response.json()
            .await
            .map_err(|e| ChatError::ApiError(format!("Failed to parse API response: {}", e)))?;

        Ok(data.results)
    }

    fn filter_results(
        &self,
        results: Vec<SearchResult>,
        min_amount: u32,
        max_amount: u32,
        preferences: &GiftPreferences,
    ) -> Vec<GiftRecommendation> {
        results.into_iter()
            .filter_map(|result| {
                // 価格範囲でフィルタリング
                if let Some(price) = self.parse_price(&result.price?) {
                    if price < min_amount || price > max_amount {
                        return None;
                    }
                }

                Some(GiftRecommendation {
                    name: result.title,
                    description: result.snippet,
                    price: result.price.and_then(|p| self.parse_price(&p)),
                    url: result.url,
                    image_url: result.image_url,
                    rating: None,
                    category: None,
                })
            })
            .take(3)
            .collect()
    }

    fn parse_price(&self, price_str: &str) -> Option<u32> {
        price_str
            .replace("円", "")
            .replace(",", "")
            .replace("¥", "")
            .trim()
            .parse()
            .ok()
    }

    fn build_search_query(
        &self,
        relationship: &str,
        min_amount: u32,
        max_amount: u32,
        preferences: &GiftPreferences,
    ) -> String {
        format!(
            "就職祝い お返し {} {}円〜{}円 {}",
            relationship,
            min_amount,
            max_amount,
            preferences.to_query_string()
        )
    }

    fn generate_cache_key(
        &self,
        relationship: &str,
        min_amount: u32,
        max_amount: u32,
        preferences: &GiftPreferences,
    ) -> String {
        format!("gift:{}:{}:{}", relationship, min_amount, max_amount)
    }
}

#[derive(Debug, Default)]
pub struct GiftPreferences {
    pub gender: Option<String>,
    pub age_range: Option<String>,
    pub categories: Vec<String>,
}

impl GiftPreferences {
    pub fn new() -> Self {
        Self::default()
    }

    fn to_query_string(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(gender) = &self.gender {
            parts.push(format!("性別:{}", gender));
        }
        
        if let Some(age) = &self.age_range {
            parts.push(format!("年代:{}", age));
        }
        
        if !self.categories.is_empty() {
            parts.push(format!("カテゴリ:{}", self.categories.join(",")));
        }
        
        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gift_recommendations() {
        let recommender = GiftRecommender::new();
        let preferences = GiftPreferences::default();
        
        let results = recommender.get_recommendations("上司", 10000, 30000, &preferences).await;
        assert!(results.is_ok());
        
        let recommendations = results.unwrap();
        assert!(!recommendations.is_empty());
        assert!(recommendations.len() <= 3);
    }
}