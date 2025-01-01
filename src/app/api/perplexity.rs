use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct PerplexityRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct PerplexityResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct PerplexityClient {
    client: Client,
    api_key: String,
    api_url: String,
}

impl PerplexityClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("PERPLEXITY_API_KEY")?;
        let api_url = env::var("PERPLEXITY_API_URL")?;

        Ok(Self {
            client: Client::new(),
            api_key,
            api_url,
        })
    }

    pub async fn search_gifts(&self, query: &str) -> Result<String> {
        let request = PerplexityRequest {
            model: "mixtral-8x7b-instruct".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "あなたはギフト推薦の専門家です。予算と状況に応じて最適なギフトを3つ提案してください。".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: query.to_string(),
                },
            ],
        };

        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<PerplexityResponse>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
} 