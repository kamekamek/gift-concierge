use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct IntentClassifier {
    patterns: HashMap<Intent, Vec<String>>,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        patterns.insert(Intent::Greeting, vec![
            "こんにちは".to_string(),
            "はじめまして".to_string(),
            "よろしく".to_string(),
            "お願いします".to_string(),
        ]);
        
        patterns.insert(Intent::AskRelationship, vec![
            "上司".to_string(),
            "先輩".to_string(),
            "友人".to_string(),
            "親戚".to_string(),
            "関係".to_string(),
            "どんな".to_string(),
            "誰".to_string(),
        ]);
        
        patterns.insert(Intent::AskBudget, vec![
            "予算".to_string(),
            "金額".to_string(),
            "円".to_string(),
            "万".to_string(),
            "いくら".to_string(),
            "どのくらい".to_string(),
        ]);
        
        patterns.insert(Intent::AskBulkGift, vec![
            "まとめて".to_string(),
            "複数".to_string(),
            "同じ".to_string(),
            "違う".to_string(),
            "何人".to_string(),
        ]);
        
        patterns.insert(Intent::AskGender, vec![
            "性別".to_string(),
            "男性".to_string(),
            "女性".to_string(),
            "男".to_string(),
            "女".to_string(),
        ]);
        
        patterns.insert(Intent::AskAge, vec![
            "年齢".to_string(),
            "歳".to_string(),
            "代".to_string(),
            "若い".to_string(),
            "高齢".to_string(),
            "いくつ".to_string(),
        ]);
        
        patterns.insert(Intent::AskManners, vec![
            "マナー".to_string(),
            "のし".to_string(),
            "礼儀".to_string(),
            "タイミング".to_string(),
            "いつまで".to_string(),
            "期限".to_string(),
        ]);
        
        Self { patterns }
    }
    
    pub fn classify(&self, text: &str) -> Intent {
        let text = text.to_lowercase();
        let mut scores = HashMap::new();
        
        for (intent, patterns) in &self.patterns {
            let score = patterns.iter()
                .filter(|pattern| text.contains(&pattern.to_lowercase()))
                .count();
            scores.insert(intent, score);
        }
        
        scores.into_iter()
            .max_by_key(|&(_, score)| score)
            .filter(|&(_, score)| score > 0)
            .map(|(intent, _)| intent.clone())
            .unwrap_or(Intent::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_classification() {
        let classifier = IntentClassifier::new();
        
        assert_eq!(classifier.classify("こんにちは、よろしくお願いします"), Intent::Greeting);
        assert_eq!(classifier.classify("上司からのプレゼントです"), Intent::AskRelationship);
        assert_eq!(classifier.classify("予算は3万円くらいです"), Intent::AskBudget);
        assert_eq!(classifier.classify("3人分まとめて購入したいです"), Intent::AskBulkGift);
        assert_eq!(classifier.classify("男性向けのギフトを探しています"), Intent::AskGender);
        assert_eq!(classifier.classify("40代の方へのプレゼントです"), Intent::AskAge);
        assert_eq!(classifier.classify("のし紙の書き方を教えてください"), Intent::AskManners);
        assert_eq!(classifier.classify("特に何も考えていません"), Intent::Unknown);
    }
} 