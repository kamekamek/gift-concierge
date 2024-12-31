use anyhow::Result;
use tokio;
use crate::app::chat::chatbot::Chatbot;
use crate::app::database::user_record::UserDatabase;
use crate::app::database::gift_cache::GiftCache;
use crate::config::config::Config;

async fn setup_test_environment() -> Result<(Chatbot, UserDatabase, GiftCache)> {
    let config = Config::new()?;
    let chatbot = Chatbot::new();
    let user_db = UserDatabase::new();
    let gift_cache = GiftCache::new(config.cache.ttl_seconds);
    
    Ok((chatbot, user_db, gift_cache))
}

#[tokio::test]
async fn test_complete_gift_recommendation_flow() -> Result<()> {
    let (chatbot, user_db, _gift_cache) = setup_test_environment().await?;
    let user_id = "test_user_1".to_string();

    // 初期メッセージ
    let response = chatbot.process_message(user_id.clone(), "こんにちは".to_string()).await?;
    assert!(response.contains("こんにちは"));

    // 関係性の入力
    let response = chatbot.process_message(user_id.clone(), "上司です".to_string()).await?;
    assert!(response.contains("予算"));

    // 予算の入力
    let response = chatbot.process_message(user_id.clone(), "3万円です".to_string()).await?;
    assert!(response.contains("複数"));

    // 人数の入力
    let response = chatbot.process_message(user_id.clone(), "1人分です".to_string()).await?;
    assert!(response.contains("性別"));

    // 性別の入力
    let response = chatbot.process_message(user_id.clone(), "男性です".to_string()).await?;
    assert!(response.contains("年齢"));

    // 年齢の入力
    let response = chatbot.process_message(user_id.clone(), "50代です".to_string()).await?;
    assert!(response.contains("おすすめ"));

    // ユーザーレコードの確認
    let user = user_db.get_user(&user_id).await;
    assert!(user.is_some());

    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_test_environment().await?;
    let user_id = "test_user_2".to_string();

    // 不正な入力
    let response = chatbot.process_message(user_id.clone(), "".to_string()).await?;
    assert!(response.contains("申し訳ありません"));

    // 不正な予算形式
    let response = chatbot.process_message(user_id.clone(), "予算はたくさん".to_string()).await?;
    assert!(response.contains("もう一度"));

    Ok(())
}

#[tokio::test]
async fn test_multilingual_support() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_test_environment().await?;
    let user_id = "test_user_3".to_string();

    // 日本語での会話
    let response = chatbot.process_message(user_id.clone(), "こんにちは".to_string()).await?;
    assert!(response.contains("こんにちは"));

    // 英語での会話
    let response = chatbot.process_message(user_id.clone(), "hello".to_string()).await?;
    assert!(response.contains("help"));

    Ok(())
}

#[tokio::test]
async fn test_cache_functionality() -> Result<()> {
    let (_chatbot, _user_db, gift_cache) = setup_test_environment().await?;
    
    // キャッシュにギフトを追加
    let gift = crate::app::database::gift_cache::CachedGift {
        name: "テストギフト".to_string(),
        description: "テスト用のギフトです".to_string(),
        price: 10000,
        category: "テスト".to_string(),
        url: None,
        cached_at: std::time::SystemTime::now(),
    };

    gift_cache.add("test_key".to_string(), gift.clone()).await?;

    // キャッシュからギフトを取得
    let cached = gift_cache.get("test_key").await;
    assert!(cached.is_some());
    assert_eq!(cached.unwrap()[0].name, "テストギフト");

    Ok(())
}

#[tokio::test]
async fn test_performance() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_test_environment().await?;
    let user_id = "test_user_4".to_string();

    use std::time::Instant;
    let start = Instant::now();

    // 複数のリクエストを同時に処理
    let mut handles = vec![];
    for i in 0..10 {
        let chatbot = chatbot.clone();
        let user_id = format!("{}_concurrent_{}", user_id, i);
        let handle = tokio::spawn(async move {
            chatbot.process_message(user_id, "こんにちは".to_string()).await
        });
        handles.push(handle);
    }

    // すべてのリクエストが完了するのを待つ
    for handle in handles {
        let result = handle.await??;
        assert!(result.contains("こんにちは"));
    }

    let duration = start.elapsed();
    // 10件の同時リクエストが3秒以内に処理されることを確認
    assert!(duration.as_secs() < 3);

    Ok(())
}

#[tokio::test]
async fn test_recommendation_quality() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_test_environment().await?;
    let user_id = "test_user_5".to_string();

    // 上司向けの高額ギフト
    let response = chatbot.process_message(
        user_id.clone(),
        "上司に5万円のギフトを探しています".to_string()
    ).await?;
    assert!(response.contains("ギフト"));

    // 推薦結果に予算内のアイテムのみが含まれることを確認
    assert!(!response.contains("60000円"));

    Ok(())
}

#[tokio::test]
async fn test_conversation_context() -> Result<()> {
    let (chatbot, _user_db, _gift_cache) = setup_test_environment().await?;
    let user_id = "test_user_6".to_string();

    // 文脈を保持しながら会話が進むことを確認
    let response1 = chatbot.process_message(user_id.clone(), "上司です".to_string()).await?;
    assert!(response1.contains("予算"));

    let response2 = chatbot.process_message(user_id.clone(), "3万円です".to_string()).await?;
    assert!(response2.contains("複数"));

    // 文脈に基づいて適切な質問が行われることを確認
    assert!(!response2.contains("関係"));

    Ok(())
} 