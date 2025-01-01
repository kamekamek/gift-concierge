use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::sync::Arc;

pub mod models;
pub mod repositories;

pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        self.pool.clone()
    }
} 