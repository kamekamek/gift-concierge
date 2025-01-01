mod app;
mod api;

use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // ロガーの初期化
    tracing_subscriber::fmt::init();

    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // アプリケーションの構築
    let app = api::create_router()
        .layer(cors);

    // サーバーアドレスの設定
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);

    // サーバーの起動
    axum::serve(listener, app).await.unwrap();
} 