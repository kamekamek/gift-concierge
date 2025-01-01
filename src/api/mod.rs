pub mod chat;
pub mod recommendations;
pub mod history;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/chat", post(chat::handle_chat))
        .route("/api/recommendations", get(recommendations::get_recommendations))
        .route("/api/history", get(history::get_chat_history))
} 