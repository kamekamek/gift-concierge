use reqwest;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct PerplexityResponse {
    results: Vec<PerplexityResult>,
}

#[derive(Debug, Deserialize)]
struct PerplexityResult {
    title: String,
    url: String,
    image: Option<String>,
}


#[derive(Debug)]
pub struct GiftRecommendation {
    name: String,
    url: String,
    image_url: Option<String>,
}

pub async fn get_gift_recommendations(relationship: &str, budget: &str) -> Result<Vec<GiftRecommendation>, Box<dyn std::error::Error>> {
    let query = format!("就職祝い お返し {} {}", relationship, budget);
    let api_key = std::env::var("PERPLEXITY_API_KEY").expect("PERPLEXITY_API_KEY not set");
    let url = format!("https://api.perplexity.ai/search?q={}&key={}", query, api_key);

    let response = reqwest::get(url).await?.text().await?;
    let data: PerplexityResponse = serde_json::from_str(&response)?;

    let mut recommendations = Vec::new();
    for result in data.results.iter().take(3) {
        recommendations.push(GiftRecommendation {
            name: result.title.clone(),
            url: result.url.clone(),
            image_url: result.image.clone(),
        });
    }

    Ok(recommendations)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_gift_recommendations() {
        let recommendations = get_gift_recommendations("上司", "1-3万円").await;
        assert!(recommendations.is_ok());
        assert!(recommendations.unwrap().len() <=3);
    }
};