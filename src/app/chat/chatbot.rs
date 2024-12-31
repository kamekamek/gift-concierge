use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: String,
    pub relationship: Option<String>,
    pub budget: Option<i32>,
    pub is_bulk_gift: Option<bool>,
    pub recipient_gender: Option<String>,
    pub recipient_age: Option<String>,
}

#[derive(Debug)]
pub struct Chatbot {
    contexts: Arc<Mutex<Vec<UserContext>>>,
}

impl Chatbot {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn process_message(&self, user_id: String, message: String) -> Result<String> {
        let mut contexts = self.contexts.lock().await;
        
        // ユーザーコンテキストの取得または作成
        let context = contexts
            .iter_mut()
            .find(|c| c.user_id == user_id)
            .unwrap_or_else(|| {
                let new_context = UserContext {
                    user_id: user_id.clone(),
                    relationship: None,
                    budget: None,
                    is_bulk_gift: None,
                    recipient_gender: None,
                    recipient_age: None,
                };
                contexts.push(new_context);
                contexts.last_mut().unwrap()
            });

        // TODO: 自然言語処理による意図解析を実装
        // TODO: 会話フローの制御を実装
        // TODO: ギフト推薦システムとの連携を実装

        Ok("申し訳ありません。現在実装中です。".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chatbot_initialization() {
        let chatbot = Chatbot::new();
        let response = chatbot
            .process_message("test_user".to_string(), "こんにちは".to_string())
            .await;
        assert!(response.is_ok());
    }
} 