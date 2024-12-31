use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedGift {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub category: String,
    pub url: Option<String>,
    pub cached_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct GiftCache {
    cache: Arc<RwLock<HashMap<String, Vec<CachedGift>>>>,
    ttl: Duration,
}

impl GiftCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<CachedGift>> {
        let cache = self.cache.read().await;
        if let Some(gifts) = cache.get(key) {
            // キャッシュの有効期限をチェック
            let now = SystemTime::now();
            let valid_gifts: Vec<CachedGift> = gifts
                .iter()
                .filter(|gift| {
                    if let Ok(elapsed) = now.duration_since(gift.cached_at) {
                        elapsed < self.ttl
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            if !valid_gifts.is_empty() {
                Some(valid_gifts)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn set(&self, key: String, gifts: Vec<CachedGift>) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.insert(key, gifts);
        Ok(())
    }

    pub async fn add(&self, key: String, gift: CachedGift) -> Result<()> {
        let mut cache = self.cache.write().await;
        if let Some(gifts) = cache.get_mut(&key) {
            gifts.push(gift);
        } else {
            cache.insert(key, vec![gift]);
        }
        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }

    pub async fn cleanup_expired(&self) -> Result<usize> {
        let mut cache = self.cache.write().await;
        let now = SystemTime::now();
        let mut removed_count = 0;

        // 期限切れのエントリを削除
        cache.retain(|_, gifts| {
            let valid_gifts: Vec<CachedGift> = gifts
                .iter()
                .filter(|gift| {
                    if let Ok(elapsed) = now.duration_since(gift.cached_at) {
                        if elapsed < self.ttl {
                            true
                        } else {
                            removed_count += 1;
                            false
                        }
                    } else {
                        removed_count += 1;
                        false
                    }
                })
                .cloned()
                .collect();

            *gifts = valid_gifts;
            !gifts.is_empty()
        });

        Ok(removed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = GiftCache::new(2); // 2秒のTTL

        let gift = CachedGift {
            name: "テストギフト".to_string(),
            description: "テスト用のギフトです".to_string(),
            price: 1000,
            category: "テスト".to_string(),
            url: None,
            cached_at: SystemTime::now(),
        };

        // キャッシュの追加
        cache.add("test_key".to_string(), gift.clone()).await.unwrap();

        // キャッシュの取得
        let cached = cache.get("test_key").await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap()[0].name, "テストギフト");

        // TTL待機
        sleep(Duration::from_secs(3)).await;

        // 期限切れの確認
        let expired = cache.get("test_key").await;
        assert!(expired.is_none());

        // クリーンアップ
        let removed = cache.cleanup_expired().await.unwrap();
        assert_eq!(removed, 1);
    }
}