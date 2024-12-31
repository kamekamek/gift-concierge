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

    fn handle_relationship(&mut self, input: &str) -> Result<String> {
        self.context.relationship = Some(input.to_string());
        self.state = ConversationState::AskGiftAmount;
        Ok("ありがとうございます。お祝いの金額の範囲を教えてください（例：1〜3万円）。".to_string())
    }

    fn handle_gift_amount(&mut self, input: &str) -> Result<String> {
        // 金額のパース処理
        let amounts: Vec<u32> = input.split('〜')
            .filter_map(|s| s.trim().replace("万円", "0000").parse().ok())
            .collect();
        
        self.context.gift_amount = match amounts.len() {
            2 => Some((amounts[0], amounts[1])),
            1 => Some((amounts[0], amounts[0])),
            _ => return Err(ChatError::InvalidInput("無効な金額形式です".to_string()))
        };

        self.state = ConversationState::AskGiftType;
        Ok("複数の方にお返しをする場合、全員同じものか、個別に選びますか？".to_string())
    }

    fn handle_gift_type(&mut self, input: &str) -> Result<String> {
        self.context.gift_type = Some(input.to_string());
        self.state = ConversationState::AskGender;
        Ok("相手の性別を教えてください（任意）。".to_string())
    }

    fn handle_gender(&mut self, input: &str) -> Result<String> {
        self.context.gender = Some(input.to_string());
        self.state = ConversationState::AskAgeRange;
        Ok("相手の年代をおおよそで教えてください（任意）。".to_string())
    }

    fn handle_age_range(&mut self, input: &str) -> Result<String> {
        self.context.age_range = Some(input.to_string());
        self.state = ConversationState::GiftRecommendation;
        Ok("ありがとうございます。最適なギフトを検索します。".to_string())
    }

    async fn generate_recommendations(&self) -> Result<String> {
        Ok("おすすめのギフトが見つかりました。".to_string())
    }
}