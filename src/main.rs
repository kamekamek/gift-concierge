use ongaeshi_concierge_bot::app::ConseljuApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = ConseljuApp::new();
    app.run().await?;
    Ok(())
} 