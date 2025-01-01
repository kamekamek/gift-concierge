use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_cors::Cors;
use actix_web_actors::ws;
use dotenv::dotenv;
use std::env;
use tracing_subscriber;

mod api;
mod app;

use crate::app::chat::websocket::WebSocketSession;
use crate::app::gift::recommendation::RecommendationService;

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocketSession::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 環境変数の読み込み
    dotenv().ok();
    
    // ロギングの初期化
    tracing_subscriber::fmt::init();

    // Perplexity APIキーの取得
    let api_key = env::var("PERPLEXITY_API_KEY")
        .expect("PERPLEXITY_API_KEY must be set");

    // レコメンデーションサービスの初期化
    let recommendation_service = web::Data::new(RecommendationService::new(api_key));

    // サーバーの起動
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(recommendation_service.clone())
            .service(web::resource("/ws").route(web::get().to(ws_index)))
            .configure(api::recommendations::config)
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
} 