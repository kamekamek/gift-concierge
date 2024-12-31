use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    pub preferred_categories: Vec<String>,
    pub excluded_categories: Vec<String>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftHistory {
    pub gift_name: String,
    pub recipient: String,
    pub price: i32,
    pub date: SystemTime,
    pub occasion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub user_id: String,
    pub preferences: UserPreference,
    pub gift_history: Vec<GiftHistory>,
    pub created_at: SystemTime,
    pub last_active: SystemTime,
}

#[derive(Debug)]
pub struct UserDatabase {
    records: Arc<RwLock<HashMap<String, UserRecord>>>,
}

impl UserDatabase {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_user(&self, user_id: String) -> Result<UserRecord> {
        let mut records = self.records.write().await;
        let now = SystemTime::now();
        
        let user = UserRecord {
            user_id: user_id.clone(),
            preferences: UserPreference {
                preferred_categories: Vec::new(),
                excluded_categories: Vec::new(),
                min_price: None,
                max_price: None,
                last_updated: now,
            },
            gift_history: Vec::new(),
            created_at: now,
            last_active: now,
        };
        
        records.insert(user_id, user.clone());
        Ok(user)
    }

    pub async fn get_user(&self, user_id: &str) -> Option<UserRecord> {
        let records = self.records.read().await;
        records.get(user_id).cloned()
    }

    pub async fn update_user(&self, user: UserRecord) -> Result<()> {
        let mut records = self.records.write().await;
        records.insert(user.user_id.clone(), user);
        Ok(())
    }

    pub async fn add_gift_history(&self, user_id: &str, history: GiftHistory) -> Result<()> {
        let mut records = self.records.write().await;
        if let Some(user) = records.get_mut(user_id) {
            user.gift_history.push(history);
            user.last_active = SystemTime::now();
        }
        Ok(())
    }

    pub async fn update_preferences(&self, user_id: &str, preferences: UserPreference) -> Result<()> {
        let mut records = self.records.write().await;
        if let Some(user) = records.get_mut(user_id) {
            user.preferences = preferences;
            user.last_active = SystemTime::now();
        }
        Ok(())
    }

    pub async fn get_recent_gifts(&self, user_id: &str, days: u64) -> Result<Vec<GiftHistory>> {
        let records = self.records.read().await;
        if let Some(user) = records.get(user_id) {
            let now = SystemTime::now();
            let duration = Duration::from_secs(days * 24 * 60 * 60);
            
            Ok(user.gift_history
                .iter()
                .filter(|history| {
                    if let Ok(elapsed) = now.duration_since(history.date) {
                        elapsed <= duration
                    } else {
                        false
                    }
                })
                .cloned()
                .collect())
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_gifts_by_recipient(&self, user_id: &str, recipient: &str) -> Result<Vec<GiftHistory>> {
        let records = self.records.read().await;
        if let Some(user) = records.get(user_id) {
            Ok(user.gift_history
                .iter()
                .filter(|history| history.recipient == recipient)
                .cloned()
                .collect())
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn cleanup_inactive_users(&self, days: u64) -> Result<usize> {
        let mut records = self.records.write().await;
        let now = SystemTime::now();
        let duration = Duration::from_secs(days * 24 * 60 * 60);
        let initial_count = records.len();
        
        records.retain(|_, user| {
            if let Ok(elapsed) = now.duration_since(user.last_active) {
                elapsed <= duration
            } else {
                false
            }
        });
        
        Ok(initial_count - records.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_user_operations() {
        let db = UserDatabase::new();
        
        // ユーザーの作成
        let user = db.create_user("test_user".to_string()).await.unwrap();
        assert_eq!(user.user_id, "test_user");
        
        // プリファレンスの更新
        let preferences = UserPreference {
            preferred_categories: vec!["電化製品".to_string()],
            excluded_categories: vec!["食品".to_string()],
            min_price: Some(5000),
            max_price: Some(30000),
            last_updated: SystemTime::now(),
        };
        db.update_preferences("test_user", preferences).await.unwrap();
        
        // ギフト履歴の追加
        let history = GiftHistory {
            gift_name: "テストギフト".to_string(),
            recipient: "テスト受取人".to_string(),
            price: 10000,
            date: SystemTime::now(),
            occasion: "誕生日".to_string(),
        };
        db.add_gift_history("test_user", history).await.unwrap();
        
        // ユーザー情報の取得
        let user = db.get_user("test_user").await.unwrap();
        assert_eq!(user.gift_history.len(), 1);
        assert_eq!(user.preferences.preferred_categories[0], "電化製品");
        
        // 非アクティブユーザーのクリーンアップ
        sleep(Duration::from_secs(1)).await;
        let removed = db.cleanup_inactive_users(0).await.unwrap();
        assert_eq!(removed, 1);
    }
} 