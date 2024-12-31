use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::app::nlp::intent_classifier::{Intent, IntentClassifier};
use crate::app::chat::conversation_handler::ConversationHandler;
use crate::app::gift::recommendation::{GiftRecommendation, RecommendationEngine};

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
    intent_classifier: IntentClassifier,
    conversation_handler: ConversationHandler,
    recommendation_engine: RecommendationEngine,
}

impl Chatbot {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(Mutex::new(Vec::new())),
            intent_classifier: IntentClassifier::new(),
            conversation_handler: ConversationHandler::new(),
            recommendation_engine: RecommendationEngine::new(),
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

        // 意図の分類
        let intent = self.intent_classifier.classify(&message);
        
        // 意図に基づく応答の生成
        let response = match intent {
            Intent::Greeting => "こんにちは！お返しの相談を承ります。".to_string(),
            Intent::AskRelationship => {
                if let Some(relationship) = self.extract_relationship(&message) {
                    context.relationship = Some(relationship);
                    self.conversation_handler.transition();
                    self.conversation_handler.get_next_question()
                } else {
                    "申し訳ありません。関係性をもう一度お聞かせいただけますか？".to_string()
                }
            }
            Intent::AskBudget => {
                if let Some(budget) = self.extract_budget(&message) {
                    context.budget = Some(budget);
                    self.conversation_handler.transition();
                    self.conversation_handler.get_next_question()
                } else {
                    "申し訳ありません。金額をもう一度お聞かせいただけますか？".to_string()
                }
            }
            Intent::AskBulkGift => {
                context.is_bulk_gift = Some(message.contains("同じ"));
                self.conversation_handler.transition();
                self.conversation_handler.get_next_question()
            }
            Intent::AskGender => {
                context.recipient_gender = Some(if message.contains("男性") {
                    "male".to_string()
                } else if message.contains("女性") {
                    "female".to_string()
                } else {
                    "unknown".to_string()
                });
                self.conversation_handler.transition();
                self.conversation_handler.get_next_question()
            }
            Intent::AskAge => {
                context.recipient_age = Some(message.to_string());
                self.conversation_handler.transition();
                
                // 必要な情報が揃った場合、ギフトを推薦
                if let (Some(relationship), Some(budget)) = (context.relationship.as_ref(), context.budget) {
                    let recommendations = self.recommendation_engine
                        .get_recommendations(
                            relationship,
                            budget,
                            context.is_bulk_gift.unwrap_or(false),
                            context.recipient_gender.as_deref(),
                            context.recipient_age.as_deref(),
                        )
                        .await?;

                    let mut response = "以下のギフトをおすすめいたします：\n\n".to_string();
                    for (i, rec) in recommendations.iter().enumerate() {
                        response.push_str(&format!(
                            "{}. {}\n   {}\n   価格: {}円\n\n",
                            i + 1,
                            rec.name,
                            rec.description,
                            rec.price
                        ));
                    }
                    response
                } else {
                    self.conversation_handler.get_next_question()
                }
            }
            Intent::AskManners => "のし紙には「御祝」と記載し、お返しは1ヶ月以内が一般的とされています。".to_string(),
            Intent::Unknown => self.conversation_handler.get_next_question(),
        };

        Ok(response)
    }

    fn extract_relationship(&self, message: &str) -> Option<String> {
        let message = message.to_lowercase();
        if message.contains("上司") {
            Some("上司".to_string())
        } else if message.contains("先輩") {
            Some("先輩".to_string())
        } else if message.contains("友人") {
            Some("友人".to_string())
        } else if message.contains("親戚") {
            Some("親戚".to_string())
        } else {
            None
        }
    }

    fn extract_budget(&self, message: &str) -> Option<i32> {
        let message = message.to_lowercase();
        if let Some(pos) = message.find("万円") {
            let start = pos.saturating_sub(10);
            let number_str: String = message[start..pos]
                .chars()
                .filter(|c| c.is_digit(10))
                .collect();
            number_str.parse::<i32>().ok().map(|n| n * 10000)
        } else if let Some(pos) = message.find("円") {
            let start = pos.saturating_sub(10);
            let number_str: String = message[start..pos]
                .chars()
                .filter(|c| c.is_digit(10))
                .collect();
            number_str.parse::<i32>().ok()
        } else {
            None
        }
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
        assert!(response.unwrap().contains("こんにちは"));
    }

    #[tokio::test]
    async fn test_relationship_extraction() {
        let chatbot = Chatbot::new();
        assert_eq!(chatbot.extract_relationship("上司です"), Some("上司".to_string()));
        assert_eq!(chatbot.extract_relationship("友人からです"), Some("友人".to_string()));
    }

    #[tokio::test]
    async fn test_budget_extraction() {
        let chatbot = Chatbot::new();
        assert_eq!(chatbot.extract_budget("3万円です"), Some(30000));
        assert_eq!(chatbot.extract_budget("30000円です"), Some(30000));
    }
} 