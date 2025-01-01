use serde::{Deserialize, Serialize};

pub struct ConversationHandler {
    // 会話の状態を管理するフィールド
}

impl ConversationHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_message(&self, message: &str) -> String {
        // TODO: 会話の文脈を理解して適切な応答を生成
        message.to_string()
    }
} 