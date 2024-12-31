use anyhow::Result;
use tokio;
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::{Semaphore, Mutex};
use my_project::app::chat::chatbot::Chatbot;
use my_project::app::database::user_record::UserDatabase;
use my_project::app::database::gift_cache::{GiftCache, CachedGift};
use my_project::config::config::Config;

const CONCURRENT_USERS: usize = 100;
const TEST_DURATION_SECS: u64 = 60;
const REQUESTS_PER_SECOND: usize = 10;

async fn setup_load_test_environment() -> Result<(Chatbot, UserDatabase, GiftCache)> {
    let config = Config::new()?;
    let chatbot = Chatbot::new();
    let user_db = UserDatabase::new();
    let gift_cache = GiftCache::new(config.cache.ttl_seconds);
    
    Ok((chatbot, user_db, gift_cache))
}

#[derive(Debug)]
struct LoadTestMetrics {
    total_requests: usize,
    successful_requests: usize,
    failed_requests: usize,
    min_response_time: Duration,
    max_response_time: Duration,
    total_response_time: Duration,
}

impl LoadTestMetrics {
    fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            min_response_time: Duration::from_secs(u64::MAX),
            max_response_time: Duration::from_secs(0),
            total_response_time: Duration::from_secs(0),
        }
    }

    fn add_request(&mut self, duration: Duration, success: bool) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        self.min_response_time = self.min_response_time.min(duration);
        self.max_response_time = self.max_response_time.max(duration);
        self.total_response_time += duration;
    }

    fn average_response_time(&self) -> Duration {
        if self.total_requests > 0 {
            self.total_response_time / self.total_requests as u32
        } else {
            Duration::from_secs(0)
        }
    }

    fn success_rate(&self) -> f64 {
        if self.total_requests > 0 {
            self.successful_requests as f64 / self.total_requests as f64 * 100.0
        } else {
            0.0
        }
    }
}

async fn simulate_user_conversation(
    chatbot: Chatbot,
    user_id: String,
    metrics: Arc<Mutex<LoadTestMetrics>>,
    rate_limiter: Arc<Semaphore>,
) -> Result<()> {
    let conversation_flow = vec![
        "こんにちは",
        "上司です",
        "3万円です",
        "1人分です",
        "男性です",
        "50代です",
    ];

    for message in conversation_flow {
        let _permit = rate_limiter.acquire().await?;
        let start = Instant::now();
        let result = chatbot.process_message(user_id.clone(), message.to_string()).await;
        let duration = start.elapsed();

        let mut metrics_guard = metrics.lock().await;
        metrics_guard.add_request(duration, result.is_ok());
        drop(metrics_guard);

        if result.is_err() {
            println!("Error in conversation for user {}: {:?}", user_id, result);
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_load() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_load_test_environment().await?;
    let metrics = Arc::new(Mutex::new(LoadTestMetrics::new()));
    let rate_limiter = Arc::new(Semaphore::new(REQUESTS_PER_SECOND));

    let start = Instant::now();
    let mut handles = vec![];

    // 同時ユーザーのシミュレーション
    for i in 0..CONCURRENT_USERS {
        let chatbot = chatbot.clone();
        let metrics = Arc::clone(&metrics);
        let rate_limiter = Arc::clone(&rate_limiter);
        let user_id = format!("load_test_user_{}", i);

        let handle = tokio::spawn(async move {
            simulate_user_conversation(chatbot, user_id, metrics, rate_limiter).await
        });
        handles.push(handle);
    }

    // すべてのユーザーシミュレーションが完了するのを待つ
    for handle in handles {
        handle.await??;
    }

    let duration = start.elapsed();
    let metrics_guard = metrics.lock().await;

    // 結果の検証
    assert!(metrics_guard.success_rate() >= 95.0, "Success rate below 95%");
    assert!(
        metrics_guard.average_response_time() < Duration::from_millis(500),
        "Average response time too high"
    );
    assert!(duration < Duration::from_secs(TEST_DURATION_SECS));

    println!("Load Test Results:");
    println!("Total Requests: {}", metrics_guard.total_requests);
    println!("Successful Requests: {}", metrics_guard.successful_requests);
    println!("Failed Requests: {}", metrics_guard.failed_requests);
    println!("Success Rate: {:.2}%", metrics_guard.success_rate());
    println!("Min Response Time: {:?}", metrics_guard.min_response_time);
    println!("Max Response Time: {:?}", metrics_guard.max_response_time);
    println!("Average Response Time: {:?}", metrics_guard.average_response_time());
    println!("Total Test Duration: {:?}", duration);

    Ok(())
}

#[tokio::test]
async fn test_sustained_load() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_load_test_environment().await?;
    let metrics = Arc::new(Mutex::new(LoadTestMetrics::new()));
    let rate_limiter = Arc::new(Semaphore::new(REQUESTS_PER_SECOND));

    let test_duration = Duration::from_secs(TEST_DURATION_SECS);
    let start = Instant::now();
    let mut handles = vec![];

    while start.elapsed() < test_duration {
        let chatbot = chatbot.clone();
        let metrics = Arc::clone(&metrics);
        let rate_limiter = Arc::clone(&rate_limiter);
        let user_id = format!("sustained_test_user_{}", handles.len());

        let handle = tokio::spawn(async move {
            simulate_user_conversation(chatbot, user_id, metrics, rate_limiter).await
        });
        handles.push(handle);

        tokio::time::sleep(Duration::from_millis(1000 / REQUESTS_PER_SECOND as u64)).await;
    }

    for handle in handles {
        handle.await??;
    }

    let metrics_guard = metrics.lock().await;

    // 結果の検証
    assert!(metrics_guard.success_rate() >= 95.0, "Sustained load success rate below 95%");
    assert!(
        metrics_guard.average_response_time() < Duration::from_millis(500),
        "Sustained load average response time too high"
    );

    println!("Sustained Load Test Results:");
    println!("Total Requests: {}", metrics_guard.total_requests);
    println!("Successful Requests: {}", metrics_guard.successful_requests);
    println!("Failed Requests: {}", metrics_guard.failed_requests);
    println!("Success Rate: {:.2}%", metrics_guard.success_rate());
    println!("Min Response Time: {:?}", metrics_guard.min_response_time);
    println!("Max Response Time: {:?}", metrics_guard.max_response_time);
    println!("Average Response Time: {:?}", metrics_guard.average_response_time());

    Ok(())
}

#[tokio::test]
async fn test_cache_performance() -> Result<()> {
    let (_chatbot, _user_db, gift_cache) = setup_load_test_environment().await?;
    let metrics = Arc::new(Mutex::new(LoadTestMetrics::new()));
    let start = Instant::now();

    // キャッシュの準備
    let test_gift = CachedGift {
        name: "テストギフト".to_string(),
        description: "テスト用のギフトです".to_string(),
        price: 10000,
        category: "テスト".to_string(),
        url: None,
        cached_at: std::time::SystemTime::now(),
    };

    // 同時にキャッシュへのアクセスをテスト
    let mut handles = vec![];
    for i in 0..1000 {
        let gift_cache = gift_cache.clone();
        let test_gift = test_gift.clone();
        let metrics = Arc::clone(&metrics);
        let key = format!("test_key_{}", i);

        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let result = gift_cache.add(key.clone(), test_gift).await;
            let duration = start.elapsed();

            let mut metrics_guard = metrics.lock().await;
            metrics_guard.add_request(duration, result.is_ok());
            drop(metrics_guard);

            // キャッシュからの読み取り
            let start = Instant::now();
            let result = gift_cache.get(&key).await;
            let duration = start.elapsed();

            let mut metrics_guard = metrics.lock().await;
            metrics_guard.add_request(duration, result.is_some());
            drop(metrics_guard);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    let duration = start.elapsed();
    let metrics_guard = metrics.lock().await;

    // 結果の検証
    assert!(metrics_guard.success_rate() >= 99.0, "Cache operation success rate below 99%");
    assert!(
        metrics_guard.average_response_time() < Duration::from_micros(1000),
        "Cache operation average response time too high"
    );

    println!("Cache Performance Test Results:");
    println!("Total Operations: {}", metrics_guard.total_requests);
    println!("Successful Operations: {}", metrics_guard.successful_requests);
    println!("Failed Operations: {}", metrics_guard.failed_requests);
    println!("Success Rate: {:.2}%", metrics_guard.success_rate());
    println!("Min Response Time: {:?}", metrics_guard.min_response_time);
    println!("Max Response Time: {:?}", metrics_guard.max_response_time);
    println!("Average Response Time: {:?}", metrics_guard.average_response_time());
    println!("Total Test Duration: {:?}", duration);

    Ok(())
} 