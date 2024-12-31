use ongaeshi_concierge_bot::app::ConseljuApp;
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ロガーの初期化
    env_logger::init();

    // アプリケーションの起動
    let app = ConseljuApp::new();
    app.run().await?;
    
    Ok(())
} 