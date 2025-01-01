use serde::{Deserialize, Serialize};

pub struct ChatBot {
    // チャットボットの状態を管理するフィールド
}

impl ChatBot {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_response(&self, input: &str) -> String {
        // TODO: OpenAI APIを使用して応答を生成
        format!("お返しのご相談ありがとうございます。「{}」についてアドバイスさせていただきます。", input)
    }
} 