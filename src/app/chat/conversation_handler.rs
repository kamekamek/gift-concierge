use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationState {
    Initial,
    AskingRelationship,
    AskingBudget,
    AskingBulkGift,
    AskingGender,
    AskingAge,
    Recommending,
    Complete,
}

#[derive(Debug)]
pub struct ConversationHandler {
    state: ConversationState,
}

impl ConversationHandler {
    pub fn new() -> Self {
        Self {
            state: ConversationState::Initial,
        }
    }

    pub fn get_next_question(&self) -> String {
        match self.state {
            ConversationState::Initial => "お返しについて相談させていただきます。まず、お祝いをくださった方との関係を教えていただけますか？（例：上司、先輩、友人、親戚など）".to_string(),
            ConversationState::AskingRelationship => "ありがとうございます。お祝いの金額はおいくらでしたか？".to_string(),
            ConversationState::AskingBudget => "複数の方へのお返しの場合、全員同じものにしますか？それとも個別に選びますか？".to_string(),
            ConversationState::AskingBulkGift => "差し支えなければ、お相手の性別を教えていただけますか？".to_string(),
            ConversationState::AskingGender => "最後に、お相手の年代を教えていただけますか？".to_string(),
            ConversationState::AskingAge => "ありがとうございます。それでは、最適なお返しを提案させていただきます。".to_string(),
            ConversationState::Recommending => "お返しの候補をご提案いたしました。他にご要望はございますか？".to_string(),
            ConversationState::Complete => "ご利用ありがとうございました。また何かございましたらお気軽にご相談ください。".to_string(),
        }
    }

    pub fn transition(&mut self) {
        self.state = match self.state {
            ConversationState::Initial => ConversationState::AskingRelationship,
            ConversationState::AskingRelationship => ConversationState::AskingBudget,
            ConversationState::AskingBudget => ConversationState::AskingBulkGift,
            ConversationState::AskingBulkGift => ConversationState::AskingGender,
            ConversationState::AskingGender => ConversationState::AskingAge,
            ConversationState::AskingAge => ConversationState::Recommending,
            ConversationState::Recommending => ConversationState::Complete,
            ConversationState::Complete => ConversationState::Initial,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_flow() {
        let mut handler = ConversationHandler::new();
        assert!(handler.get_next_question().contains("関係"));
        
        handler.transition();
        assert!(handler.get_next_question().contains("金額"));
        
        handler.transition();
        assert!(handler.get_next_question().contains("全員同じ"));
    }
} 