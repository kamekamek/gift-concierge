use crate::nlp::IntentClassifier;
use crate::chat::ConversationHandler;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContext {
    relationship: Option<String>,
    gift_amount: Option<(u32, u32)>,
    gift_type: Option<String>,
    gender: Option<String>,
    age_range: Option<String>,
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
    conversation_handler: ConversationHandler,
    intent_classifier: IntentClassifier,
    context: UserContext,
    state: ConversationState,
}

#[async_trait]
impl Chatbot {
    pub fn new() -> Self {
        Self {
            conversation_handler: ConversationHandler::new(),
            intent_classifier: IntentClassifier::new(),
            context: UserContext {
                relationship: None,
                gift_amount: None,
                gift_type: None,
                gender: None,
                age_range: None,
            },
            state: ConversationState::Initial,
        }
    }

    pub async fn process_message(&mut self, message: String) -> Result<String, ChatError> {
        match self.state {
            ConversationState::Initial => Ok(self.start_conversation()),
            ConversationState::AskRelationship => Ok(self.handle_relationship(&message)),
            ConversationState::AskGiftAmount => Ok(self.handle_gift_amount(&message)),
            _ => Ok("処理を継続します...".to_string()),
        }
    }

    fn start_conversation(&mut self) -> String {
        self.state = ConversationState::AskRelationship;
        "就職祝いのお返しについて、お手伝いさせていただきます。まず、お祝いをくれた方との関係性を教えてください。".to_string()
    }
}