use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use std::sync::Arc;

use crate::app::gift::recommendation::{GiftRecommender, GiftRequest, GiftRecommendation};

#[derive(Clone)]
pub struct AppState {
    recommender: Arc<GiftRecommender>,
}

impl AppState {
    pub fn new(perplexity_api_key: String) -> Self {
        Self {
            recommender: Arc::new(GiftRecommender::new(perplexity_api_key)),
        }
    }
}

pub fn gift_routes() -> Router<AppState> {
    Router::new()
        .route("/recommendations", post(get_recommendations))
}

async fn get_recommendations(
    State(state): State<AppState>,
    Json(request): Json<GiftRequest>,
) -> Json<Vec<GiftRecommendation>> {
    match state.recommender.get_recommendations(request).await {
        Ok(recommendations) => Json(recommendations),
        Err(e) => {
            tracing::error!("Failed to get recommendations: {:?}", e);
            Json(vec![]) // エラー時は空の配列を返す
        }
    }
} 