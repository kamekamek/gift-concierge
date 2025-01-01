use axum::{
    extract::Query,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::app::gift::recommendation::GiftRecommender;

#[derive(Debug, Deserialize)]
pub struct RecommendationParams {
    budget: Option<i32>,
    occasion: Option<String>,
    age: Option<i32>,
    gender: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Gift {
    name: String,
    price: i32,
    description: String,
    image_url: Option<String>,
}

pub async fn get_recommendations(
    Query(params): Query<RecommendationParams>,
) -> impl IntoResponse {
    let recommender = GiftRecommender::new();
    
    match recommender.get_recommendations(
        params.budget,
        params.occasion.as_deref(),
        params.age,
        params.gender.as_deref(),
    ).await {
        Ok(gifts) => (StatusCode::OK, Json(gifts)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(vec![]),
        ),
    }
} 