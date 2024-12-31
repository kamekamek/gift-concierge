use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Intent {
    Greeting,
    AskRelationship,
    AskBudget,
    AskBulkGift,
    AskGender,
    AskAge,
    AskManners,
    Unknown,
}

#[derive(Debug)]
pub struct IntentClassifier;

impl IntentClassifier {
    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, text: &str) -> Intent {
        let text = text.to_lowercase();
        
        // 簡単なキーワードベースの分類
        if text.contains("こんにちは") || text.contains("はじめまして") {
            Intent::Greeting
        } else if text.contains("関係") || text.contains("上司") || text.contains("先輩") || text.contains("友人") {
            Intent::AskRelationship
        } else if text.contains("金額") || text.contains("予算") || text.contains("円") {
            Intent::AskBudget
        } else if text.contains("全員") || text.contains("個別") || text.contains("それぞれ") {
            Intent::AskBulkGift
        } else if text.contains("性別") || text.contains("男性") || text.contains("女性") {
            Intent::AskGender
        } else if text.contains("年齢") || text.contains("代") {
            Intent::AskAge
        } else if text.contains("マナー") || text.contains("時期") || text.contains("のし") {
            Intent::AskManners
        } else {
            Intent::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_classification() {
        let classifier = IntentClassifier::new();
        
        assert!(matches!(classifier.classify("こんにちは"), Intent::Greeting));
        assert!(matches!(classifier.classify("上司との関係です"), Intent::AskRelationship));
        assert!(matches!(classifier.classify("予算は3万円です"), Intent::AskBudget));
        assert!(matches!(classifier.classify("全員同じものにしたいです"), Intent::AskBulkGift));
        assert!(matches!(classifier.classify("男性です"), Intent::AskGender));
        assert!(matches!(classifier.classify("40代です"), Intent::AskAge));
        assert!(matches!(classifier.classify("のしの書き方は？"), Intent::AskManners));
    }
} 