#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_flow() {
        let chatbot = Chatbot::new();
        assert_eq!(chatbot.start_conversation(), "こんにちは、お返しコンシェルジュBOTです。どのようにお手伝いしましょうか？");
        assert_eq!(chatbot.handle_input("就職祝いのお返しについて知りたい"), "お祝いをくれた相手との関係性を教えてください。");
        // 他の会話フローをテスト
    }

    #[test]
    fn test_intent_classification() {
        let classifier = IntentClassifier::new();
        assert_eq!(classifier.classify("就職祝いのお返しに何がいい？"), Intent::GiftRecommendation);
        assert_eq!(classifier.classify("お返しのマナーを知りたい"), Intent::MannerQuestion);
        // 他の意図分類をテスト
    }
};