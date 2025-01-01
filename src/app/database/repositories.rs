use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use time::OffsetDateTime;

use super::models::{ChatHistory, GiftRecommendation, UserPreference};

pub struct ChatHistoryRepository {
    pool: Arc<PgPool>,
}

impl ChatHistoryRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn save(&self, user_id: &str, message: &str, response: &str) -> Result<ChatHistory> {
        let record = sqlx::query_as!(
            ChatHistory,
            r#"
            INSERT INTO chat_history (user_id, message, response, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, message, response, created_at
            "#,
            user_id,
            message,
            response,
            OffsetDateTime::now_utc()
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_history(&self, user_id: &str, limit: i32) -> Result<Vec<ChatHistory>> {
        let records = sqlx::query_as!(
            ChatHistory,
            r#"
            SELECT id, user_id, message, response, created_at
            FROM chat_history
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(records)
    }
}

pub struct GiftRecommendationRepository {
    pool: Arc<PgPool>,
}

impl GiftRecommendationRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn save(&self, recommendation: &GiftRecommendation) -> Result<GiftRecommendation> {
        let record = sqlx::query_as!(
            GiftRecommendation,
            r#"
            INSERT INTO gift_recommendations (
                name, price, description, image_url, category, rating, source, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, price, description, image_url, category, rating, source, created_at
            "#,
            recommendation.name,
            recommendation.price,
            recommendation.description,
            recommendation.image_url,
            recommendation.category,
            recommendation.rating,
            recommendation.source,
            OffsetDateTime::now_utc()
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(record)
    }

    pub async fn find_by_criteria(
        &self,
        min_price: Option<i32>,
        max_price: Option<i32>,
        category: Option<&str>,
    ) -> Result<Vec<GiftRecommendation>> {
        let records = sqlx::query_as!(
            GiftRecommendation,
            r#"
            SELECT id, name, price, description, image_url, category, rating, source, created_at
            FROM gift_recommendations
            WHERE ($1::int IS NULL OR price >= $1)
            AND ($2::int IS NULL OR price <= $2)
            AND ($3::text IS NULL OR category = $3)
            ORDER BY rating DESC
            LIMIT 10
            "#,
            min_price,
            max_price,
            category
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(records)
    }
}

pub struct UserPreferenceRepository {
    pool: Arc<PgPool>,
}

impl UserPreferenceRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn save(&self, preference: &UserPreference) -> Result<UserPreference> {
        let record = sqlx::query_as!(
            UserPreference,
            r#"
            INSERT INTO user_preferences (
                user_id, budget_min, budget_max, preferred_categories, updated_at
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id) DO UPDATE
            SET budget_min = EXCLUDED.budget_min,
                budget_max = EXCLUDED.budget_max,
                preferred_categories = EXCLUDED.preferred_categories,
                updated_at = EXCLUDED.updated_at
            RETURNING id, user_id, budget_min, budget_max, preferred_categories, updated_at
            "#,
            preference.user_id,
            preference.budget_min,
            preference.budget_max,
            &preference.preferred_categories,
            OffsetDateTime::now_utc()
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(record)
    }

    pub async fn find_by_user_id(&self, user_id: &str) -> Result<Option<UserPreference>> {
        let record = sqlx::query_as!(
            UserPreference,
            r#"
            SELECT id, user_id, budget_min, budget_max, preferred_categories, updated_at
            FROM user_preferences
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(record)
    }
} 