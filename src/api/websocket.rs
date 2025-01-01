use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::app::chat::chatbot::ChatBot;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    user_id: String,
    message: String,
    message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    UserMessage,
    BotResponse,
    Typing,
    Error,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let chatbot = Arc::new(ChatBot::new());

    // メッセージ受信ループ
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            match serde_json::from_str::<ChatMessage>(&text) {
                Ok(chat_message) => {
                    // タイピング状態を送信
                    let typing = ChatMessage {
                        user_id: chat_message.user_id.clone(),
                        message: "...".to_string(),
                        message_type: MessageType::Typing,
                    };
                    if let Ok(typing_text) = serde_json::to_string(&typing) {
                        let _ = sender.send(Message::Text(typing_text)).await;
                    }

                    // チャットボットで処理
                    match chatbot.process_message(&chat_message.message).await {
                        Ok(bot_response) => {
                            let response = ChatMessage {
                                user_id: chat_message.user_id,
                                message: bot_response,
                                message_type: MessageType::BotResponse,
                            };

                            if let Ok(response_text) = serde_json::to_string(&response) {
                                if sender.send(Message::Text(response_text)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            let error_message = ChatMessage {
                                user_id: "system".to_string(),
                                message: "メッセージの処理中にエラーが発生しました".to_string(),
                                message_type: MessageType::Error,
                            };

                            if let Ok(error_text) = serde_json::to_string(&error_message) {
                                if sender.send(Message::Text(error_text)).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    let error_message = ChatMessage {
                        user_id: "system".to_string(),
                        message: "無効なメッセージ形式です".to_string(),
                        message_type: MessageType::Error,
                    };

                    if let Ok(error_text) = serde_json::to_string(&error_message) {
                        if sender.send(Message::Text(error_text)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
} 