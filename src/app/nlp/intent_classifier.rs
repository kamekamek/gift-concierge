use std::collections::HashMap;
use log::debug;

#[derive(Debug)]
pub struct IntentClassifier {
    patterns: HashMap<String, Vec<String>>,
    context_patterns: HashMap<String, Vec<String>>,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut classifier = Self {
            patterns: HashMap::new(),
            context_patterns: HashMap::new(),
        };
        classifier.initialize_patterns();
        classifier
    }

    fn initialize_patterns(&mut self) {
        // 基本的な意図パターン
        self.patterns.insert(
            "relationship".to_string(),
            vec![
                "上司".to_string(), 
                "先輩".to_string(), 
                "友人".to_string(), 
                "親戚".to_string(),
                "同僚".to_string(),
                "後輩".to_string(),
            ]
        );

        // 金額関連パターン
        self.patterns.insert(
            "amount".to_string(),
            vec![
                "万円".to_string(),
                "円".to_string(),
                "予算".to_string(),
                "金額".to_string(),
            ]
        );

        // ギフトタイプパターン
        self.patterns.insert(
            "gift_type".to_string(),
            vec![
                "同じ".to_string(),
                "個別".to_string(),
                "それぞれ".to_string(),
                "一緒".to_string(),
            ]
        );

        // コンテキスト依存パターン
        self.context_patterns.insert(
            "confirmation".to_string(),
            vec![
                "はい".to_string(),
                "そうです".to_string(),
                "違います".to_string(),
                "いいえ".to_string(),
            ]
        );
    }

    pub fn classify(&self, input: &str, context: Option<&str>) -> Option<String> {
        debug!("Classifying input: {} with context: {:?}", input, context);

        // コンテキストベースの分類
        if let Some(ctx) = context {
            if let Some(intent) = self.classify_with_context(input, ctx) {
                return Some(intent);
            }
        }

        // 一般的な意図分類
        for (intent, keywords) in &self.patterns {
            for keyword in keywords {
                if input.contains(keyword) {
                    debug!("Matched intent: {} with keyword: {}", intent, keyword);
                    return Some(intent.to_string());
                }
            }
        }

        debug!("No intent matched for input");
        None
    }

    fn classify_with_context(&self, input: &str, context: &str) -> Option<String> {
        match context {
            "asking_amount" => self.classify_amount(input),
            "asking_gift_type" => self.classify_gift_type(input),
            _ => None,
        }
    }

    fn classify_amount(&self, input: &str) -> Option<String> {
        if input.contains("万円") || input.contains("円") {
            Some("amount".to_string())
        } else {
            None
        }
    }

    fn classify_gift_type(&self, input: &str) -> Option<String> {
        if input.contains("同じ") || input.contains("個別") {
            Some("gift_type".to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_classification() {
        let classifier = IntentClassifier::new();
        assert_eq!(classifier.classify("上司からのお祝い", None), Some("relationship".to_string()));
        assert_eq!(classifier.classify("友人に贈りたい", None), Some("relationship".to_string()));
    }

    #[test]
    fn test_amount_classification() {
        let classifier = IntentClassifier::new();
        assert_eq!(classifier.classify("3万円くらい", Some("asking_amount")), Some("amount".to_string()));
        assert_eq!(classifier.classify("30000円", Some("asking_amount")), Some("amount".to_string()));
    }

    #[test]
    fn test_gift_type_classification() {
        let classifier = IntentClassifier::new();
        assert_eq!(classifier.classify("全員同じものを贈りたい", Some("asking_gift_type")), Some("gift_type".to_string()));
        assert_eq!(classifier.classify("個別に選びたい", Some("asking_gift_type")), Some("gift_type".to_string()));
    }
}