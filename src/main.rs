mod app {
    pub mod chat {
        pub mod chatbot;
        pub mod conversation_handler;
    }
}

use app::chat::chatbot::Chatbot;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // ロギングの初期化
    tracing_subscriber::fmt::init();

    // チャットボットの初期化
    let chatbot = Chatbot::new();

    // テスト用のメッセージ処理
    let response = chatbot
        .process_message("test_user".to_string(), "こんにちは".to_string())
        .await?;

    println!("Bot: {}", response);

    Ok(())
} 