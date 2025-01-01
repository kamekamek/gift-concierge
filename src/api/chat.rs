use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::app::chat::chatbot::ChatBot;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    user_id: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    message: String,
    recommendations: Option<Vec<String>>,
}

pub async fn handle_chat(
    Json(request): Json<ChatRequest>,
) -> impl IntoResponse {
    let chatbot = ChatBot::new();
    
    match chatbot.process_message(&request.message).await {
        Ok(response) => {
            let chat_response = ChatResponse {
                message: response,
                recommendations: None, // 後で実装
            };
            (StatusCode::OK, Json(chat_response))
        }
        Err(_) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChatResponse {
                    message: "内部エラーが発生しました".to_string(),
                    recommendations: None,
                }),
            )
        }
    }
} 