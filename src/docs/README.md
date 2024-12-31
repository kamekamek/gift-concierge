私は、プロジェクト構造の`src/app/chat/chatbot.rs`の実装を行います。以下にRustでのチャットボット実装を提示します：

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use perplexity_api::PerplexityClient;

#[derive(Debug, Serialize, Deserialize)]
struct UserContext {
    relationship: Option<String>,
    gift_amount: Option<(u32, u32)>,
    gift_type: Option<String>,
    gender: Option<String>,
    age_range: Option<String>,
}

#[derive(Debug)]
enum ConversationState {
    Initial,
    AskRelationship,
    AskGiftAmount,
    AskGiftType,
    AskGender,
    AskAgeRange,
    GiftRecommendation,
}

struct ChatBot {
    context: UserContext,
    state: ConversationState,
    perplexity_client: PerplexityClient,
}

#[async_trait]
impl ChatBot {
    async fn new() -> Self {
        ChatBot {
            context: UserContext {
                relationship: None,
                gift_amount: None,
                gift_type: None,
                gender: None,
                age_range: None,
            },
            state: ConversationState::Initial,
            perplexity_client: PerplexityClient::new(),
        }
    }

    async fn process_input(&mut self, input: &str) -> String {
        match self.state {
            ConversationState::Initial => self.start_conversation(),
            ConversationState::AskRelationship => self.handle_relationship(input),
            ConversationState::AskGiftAmount => self.handle_gift_amount(input),
            ConversationState::AskGiftType => self.handle_gift_type(input),
            ConversationState::AskGender => self.handle_gender(input),
            ConversationState::AskAgeRange => self.handle_age_range(input),
            ConversationState::GiftRecommendation => self.generate_gift_recommendations().await,
        }
    }

    fn start_conversation(&mut self) -> String {
        self.state = ConversationState::AskRelationship;
        "就職祝いのお返しについて、お手伝いさせていただきます。まず、お祝いをくれた方との関係性を教えてください。".to_string()
    }

    fn handle_relationship(&mut self, input: &str) -> String {
        self.context.relationship = Some(input.to_string());
        self.state = ConversationState::AskGiftAmount;
        "ありがとうございます。お祝いの金額の範囲を教えてください（例：1〜3万円）。".to_string()
    }

    fn handle_gift_amount(&mut self, input: &str) -> String {
        // 金額範囲のパース処理
        let amounts: Vec<u32> = input.split('〜')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        
        self.context.gift_amount = match amounts.len() {
            2 => Some((amounts[0], amounts[1])),
            1 => Some((0, amounts[0])),
            _ => None
        };

        self.state = ConversationState::AskGiftType;
        "複数の方にお返しをする場合、全員同じものか、個別に選びますか？".to_string()
    }

    fn handle_gift_type(&mut self, input: &str) -> String {
        self.context.gift_type = Some(input.to_string());
        self.state = ConversationState::AskGender;
        "相手の性別を教えてください（任意）。".to_string()
    }

    fn handle_gender(&mut self, input: &str) -> String {
        self.context.gender = Some(input.to_string());
        self.state = ConversationState::AskAgeRange;
        "相手の年代をおおよそで教えてください（任意）。".to_string()
    }

    fn handle_age_range(&mut self, input: &str) -> String {
        self.context.age_range