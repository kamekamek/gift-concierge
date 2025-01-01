use std::error::Error;
use tracing::info;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ChatBot {
    conversation_state: Arc<Mutex<ConversationState>>,
}

#[derive(Debug, Default)]
struct ConversationState {
    current_step: ConversationStep,
    gift_params: GiftParams,
}

#[derive(Debug, Default)]
enum ConversationStep {
    #[default]
    Initial,
    AskingBudget,
    AskingOccasion,
    AskingAge,
    AskingGender,
    Recommending,
}

#[derive(Debug, Default)]
struct GiftParams {
    budget: Option<i32>,
    occasion: Option<String>,
    age: Option<i32>,
    gender: Option<String>,
}

impl ChatBot {
    pub fn new() -> Self {
        Self {
            conversation_state: Arc::new(Mutex::new(ConversationState::default())),
        }
    }

    pub async fn process_message(&self, message: &str) -> Result<String> {
        let mut state = self.conversation_state.lock().await;
        
        match state.current_step {
            ConversationStep::Initial => {
                state.current_step = ConversationStep::AskingBudget;
                Ok("ギフト選びのお手伝いをさせていただきます。予算はおいくらくらいでしょうか？".to_string())
            }
            ConversationStep::AskingBudget => {
                if let Ok(budget) = message.trim().replace("円", "").replace(",", "").parse::<i32>() {
                    state.gift_params.budget = Some(budget);
                    state.current_step = ConversationStep::AskingOccasion;
                    Ok("どのようなお返しでしょうか？（例：就職祝い、結婚祝い、など）".to_string())
                } else {
                    Ok("申し訳ありません。予算を数字で入力してください。".to_string())
                }
            }
            ConversationStep::AskingOccasion => {
                state.gift_params.occasion = Some(message.to_string());
                state.current_step = ConversationStep::AskingAge;
                Ok("お相手の年齢層を教えていただけますか？".to_string())
            }
            ConversationStep::AskingAge => {
                if let Ok(age) = message.trim().replace("歳", "").replace("代", "").parse::<i32>() {
                    state.gift_params.age = Some(age);
                    state.current_step = ConversationStep::AskingGender;
                    Ok("お相手の性別を教えていただけますか？".to_string())
                } else {
                    Ok("申し訳ありません。年齢を数字で入力してください。".to_string())
                }
            }
            ConversationStep::AskingGender => {
                state.gift_params.gender = Some(message.to_string());
                state.current_step = ConversationStep::Recommending;
                
                // ここでギフト推薦ロジックを呼び出す
                Ok("ご要望に合わせたギフトをお探しします。少々お待ちください...".to_string())
            }
            ConversationStep::Recommending => {
                state.current_step = ConversationStep::Initial;
                Ok("新しいギフト相談を始めましょう。どのようなお返しをお探しですか？".to_string())
            }
        }
    }
} 