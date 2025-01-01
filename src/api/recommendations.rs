use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::app::gift::recommendation::{RecommendationService, GiftInput};

#[derive(Debug, Deserialize)]
pub struct RecommendationRequest {
    gift_type: String,
    price_range: String,
    relationship: String,
    event_type: String,
    additional_notes: Option<String>,
}

pub async fn get_recommendations(
    data: web::Json<RecommendationRequest>,
    recommendation_service: web::Data<RecommendationService>,
) -> Result<HttpResponse> {
    let input = GiftInput {
        gift_type: data.gift_type.clone(),
        price_range: data.price_range.clone(),
        relationship: data.relationship.clone(),
        event_type: data.event_type.clone(),
        additional_notes: data.additional_notes.clone(),
    };

    match recommendation_service.generate_recommendations(input).await {
        Ok(recommendations) => Ok(HttpResponse::Ok().json(recommendations)),
        Err(e) => {
            eprintln!("推奨生成エラー: {}", e);
            Ok(HttpResponse::InternalServerError().json("推奨の生成中にエラーが発生しました"))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/recommendations")
            .route(web::post().to(get_recommendations))
    );
} 