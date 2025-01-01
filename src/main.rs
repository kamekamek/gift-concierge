mod app;
mod api;

use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

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
    println!("Server running on http://{}", addr);

    // サーバーの起動
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
} 