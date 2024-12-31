use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub enum ConversationState {
    Initial,
    AskingRelationship,
    AskingBudget,
    AskingBulkGift,
    AskingGender,
    AskingAge,
    Complete,
}

#[derive(Debug)]
pub struct ConversationHandler {
    state: ConversationState,
    step_counter: AtomicUsize,
}

impl ConversationHandler {
    pub fn new() -> Self {
        Self {
            state: ConversationState::Initial,
            step_counter: AtomicUsize::new(0),
        }
    }

    pub fn transition(&mut self) {
        self.state = match self.state {
            ConversationState::Initial => ConversationState::AskingRelationship,
            ConversationState::AskingRelationship => ConversationState::AskingBudget,
            ConversationState::AskingBudget => ConversationState::AskingBulkGift,
            ConversationState::AskingBulkGift => ConversationState::AskingGender,
            ConversationState::AskingGender => ConversationState::AskingAge,
            ConversationState::AskingAge => ConversationState::Complete,
            ConversationState::Complete => ConversationState::Complete,
        };
        self.step_counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_next_question(&self) -> String {
        match self.state {
            ConversationState::Initial => {
                "ギフト選びのお手伝いをさせていただきます。まず、贈り主様との関係を教えていただけますか？\n\
                 例：上司、先輩、友人、親戚など".to_string()
            }
            ConversationState::AskingRelationship => {
                "ご予算はどのくらいをお考えでしょうか？\n\
                 例：3万円、5000円など".to_string()
            }
            ConversationState::AskingBudget => {
                "複数の方へのギフトをお探しでしょうか？それとも1名様分でしょうか？\n\
                 例：3人分まとめて、1人分".to_string()
            }
            ConversationState::AskingBulkGift => {
                "受け取られる方の性別を教えていただけますか？\n\
                 例：男性、女性".to_string()
            }
            ConversationState::AskingGender => {
                "受け取られる方の年齢層を教えていただけますか？\n\
                 例：20代、30代、40代など".to_string()
            }
            ConversationState::AskingAge => {
                "ありがとうございます。これらの情報を基に、最適なギフトをご提案させていただきます。".to_string()
            }
            ConversationState::Complete => {
                "他にご要望やご質問はございますか？\n\
                 例：のし紙の書き方、マナーについてなど".to_string()
            }
        }
    }

    pub fn get_current_state(&self) -> &ConversationState {
        &self.state
    }

    pub fn get_step_count(&self) -> usize {
        self.step_counter.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_flow() {
        let mut handler = ConversationHandler::new();
        
        // 初期状態
        assert!(matches!(handler.get_current_state(), ConversationState::Initial));
        assert!(handler.get_next_question().contains("関係"));
        
        // 関係性の質問へ
        handler.transition();
        assert!(matches!(handler.get_current_state(), ConversationState::AskingRelationship));
        assert!(handler.get_next_question().contains("予算"));
        
        // 予算の質問へ
        handler.transition();
        assert!(matches!(handler.get_current_state(), ConversationState::AskingBudget));
        assert!(handler.get_next_question().contains("複数"));
        
        // まとめ買いの質問へ
        handler.transition();
        assert!(matches!(handler.get_current_state(), ConversationState::AskingBulkGift));
        assert!(handler.get_next_question().contains("性別"));
        
        // 性別の質問へ
        handler.transition();
        assert!(matches!(handler.get_current_state(), ConversationState::AskingGender));
        assert!(handler.get_next_question().contains("年齢"));
        
        // 年齢の質問へ
        handler.transition();
        assert!(matches!(handler.get_current_state(), ConversationState::AskingAge));
        assert!(handler.get_next_question().contains("ありがとうございます"));
        
        // 完了状態へ
        handler.transition();
        assert!(matches!(handler.get_current_state(), ConversationState::Complete));
        assert!(handler.get_next_question().contains("他に"));
        
        // ステップカウントの確認
        assert_eq!(handler.get_step_count(), 6);
    }
} 