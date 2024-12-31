use crate::chat::Chatbot;
use crate::gift::GiftRecommendation;
use crate::database::UserRecords;

pub struct ConseljuApp {
    chatbot: Chatbot,
    recommendation: GiftRecommendation,
    user_records: UserRecords,
}

impl ConseljuApp {
    pub fn new() -> Self {
        Self {
            chatbot: Chatbot::new(),
            recommendation: GiftRecommendation::new(),
            user_records: UserRecords::new(),
        }
    }

    pub async fn run() -> Result<(), Error> {
        // アプリケーションのメインループ実装
        println!("お返しコンシェルジュBOTを起動しています...");
        Ok(())
    }
} 