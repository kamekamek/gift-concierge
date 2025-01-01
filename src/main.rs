use axum::Router;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;

mod app;
mod api;

#[tokio::main]
async fn main() {
    // 環境変数の読み込み
    dotenv().ok();
    
    // ロギングの初期化
    tracing_subscriber::fmt::init();

    // Perplexity API キーの取得
    let perplexity_api_key = env::var("PERPLEXITY_API_KEY")
        .expect("PERPLEXITY_API_KEY must be set");

    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // アプリケーション状態の初期化
    let app_state = api::gift::AppState::new(perplexity_api_key);

    // ルーターの設定
    let app = Router::new()
        .merge(api::gift::gift_routes())
        .layer(cors)
        .with_state(app_state);

    // サーバーの起動
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap(),
        app
    )
    .await
    .unwrap();
} 