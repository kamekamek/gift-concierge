use axum::{
    extract::Query,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HistoryParams {
    user_id: String,
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    user_id: String,
    message: String,
    response: String,
    timestamp: i64,
}

pub async fn get_chat_history(
    Query(params): Query<HistoryParams>,
) -> impl IntoResponse {
    // TODO: データベースからの履歴取得を実装
    let _limit = params.limit.unwrap_or(10);
    let messages: Vec<ChatMessage> = vec![]; // 仮の空実装
    
    (StatusCode::OK, Json(messages))
} 