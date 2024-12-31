use crate::app::error::{ChatError, Result};
use crate::app::nlp::IntentClassifier;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContext {
    pub relationship: Option<String>,
    pub gift_amount: Option<(u32, u32)>,
    pub gift_type: Option<String>,
    pub gender: Option<String>,
    pub age_range: Option<String>,
}

#[derive(Debug)]
pub enum ConversationState {
    Initial,
    AskRelationship,
    AskGiftAmount,
    AskGiftType,
    AskGender,
    AskAgeRange,
    GiftRecommendation,
}

pub struct Chatbot {
    context: UserContext,
    state: ConversationState,
    intent_classifier: IntentClassifier,
}

impl Chatbot {
    pub fn new() -> Self {
        Self {
            context: UserContext {
                relationship: None,
                gift_amount: None,
                gift_type: None,
                gender: None,
                age_range: None,
            },
            state: ConversationState::Initial,
            intent_classifier: IntentClassifier::new(),
        }
    }

    pub async fn process_message(&mut self, message: &str) -> Result<String> {
        info!("Processing message: {}", message);
        
        if message.trim().is_empty() {
            return Err(ChatError::InvalidInput("メッセージが空です".to_string()));
        }

        let response = match self.state {
            ConversationState::Initial => self.start_conversation(),
            ConversationState::AskRelationship => self.handle_relationship(message),
            ConversationState::AskGiftAmount => self.handle_gift_amount(message),
            ConversationState::AskGiftType => self.handle_gift_type(message),
            ConversationState::AskGender => self.handle_gender(message),
            ConversationState::AskAgeRange => self.handle_age_range(message),
            ConversationState::GiftRecommendation => self.generate_recommendations().await,
        };

        match response {
            Ok(msg) => {
                info!("Generated response: {}", msg);
                Ok(msg)
            }
            Err(e) => {
                error!("Error processing message: {:?}", e);
                Err(e)
            }
        }
    }

    fn start_conversation(&mut self) -> Result<String> {
        self.state = ConversationState::AskRelationship;
        Ok("就職祝いのお返しについて、お手伝いさせていただきます。まず、お祝いをくれた方との関係性を教えてください。".to_string())
    }

    // 他のハンドラメソッドも同様に実装...
}