pub mod chat;
pub mod gift;
pub mod database;
pub mod nlp;

pub use crate::app::chat::Chatbot;
pub use crate::app::gift::GiftRecommendation;
pub use crate::app::database::UserRecords;

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

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("お返しコンシェルジュBOTを起動しています...");
        Ok(())
    }
} 