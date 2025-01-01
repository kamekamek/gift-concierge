use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatHistory {
    pub id: i32,
    pub user_id: String,
    pub message: String,
    pub response: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GiftRecommendation {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: String,
    pub image_url: Option<String>,
    pub category: String,
    pub rating: f32,
    pub source: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserPreference {
    pub id: i32,
    pub user_id: String,
    pub budget_min: Option<i32>,
    pub budget_max: Option<i32>,
    pub preferred_categories: Vec<String>,
    pub updated_at: OffsetDateTime,
} 