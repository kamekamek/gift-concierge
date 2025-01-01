mod app {
    pub mod chat {
        pub mod chatbot;
        pub mod conversation_handler;
    }
    pub mod nlp {
        pub mod intent_classifier;
    }
    pub mod gift {
        pub mod recommendation;
    }
}

use app::chat::chatbot::Chatbot;
use anyhow::Result;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ロギングの初期化
    tracing_subscriber::fmt::init();

    // チャットボットの初期化
    let chatbot = Chatbot::new();

    println!("Bot: こんにちは！お返しの相談を承ります。");
    
    loop {
        print!("You: ");
        io::stdout().flush()?;  // プロンプトを即座に表示

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        // 終了条件
        if input.trim().eq_ignore_ascii_case("quit") || 
           input.trim().eq_ignore_ascii_case("exit") {
            println!("Bot: ご利用ありがとうございました。");
            break;
        }

        // ボットの応答を取得して表示
        let response = chatbot.process_message("user1".to_string(), input).await?;
        println!("Bot: {}", response);
    }

    Ok(())
} 