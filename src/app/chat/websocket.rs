use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use serde_json::{json, Value};
use tracing::info;

use super::chatbot::ChatBot;
use super::conversation_handler::ConversationHandler;

pub struct WebSocketSession {
    chatbot: ChatBot,
    conversation_handler: ConversationHandler,
}

impl WebSocketSession {
    pub fn new() -> Self {
        Self {
            chatbot: ChatBot::new(),
            conversation_handler: ConversationHandler::new(),
        }
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                info!("Received message: {}", text);
                
                // メッセージをJSONとしてパース
                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    if let Some(message) = value.get("message").and_then(Value::as_str) {
                        // 会話を処理
                        let response = self.conversation_handler.process_message(message);
                        
                        // チャットボットからの応答を生成
                        let bot_response = self.chatbot.generate_response(&response);
                        
                        // レスポンスをJSON形式で送信
                        let response_json = json!({
                            "user_id": "bot",
                            "message": bot_response,
                            "message_type": "bot_response"
                        });
                        
                        ctx.text(response_json.to_string());
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {}
            Ok(ws::Message::Nop) => {}
            Err(e) => {
                info!("Error: {:?}", e);
                ctx.stop();
            }
        }
    }
} 