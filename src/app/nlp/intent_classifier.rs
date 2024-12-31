use std::collections::HashMap;

pub struct IntentClassifier {
    patterns: HashMap<String, Vec<String>>,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert("relationship".to_string(), vec!["上司", "先輩", "友人", "親戚"]);
        patterns.insert("amount".to_string(), vec!["1万円", "3万円", "5万円"]);
        patterns.insert("individual".to_string(), vec!["個別", "全員同じ"]);
        Self { patterns }
    }

    pub fn classify(&self, input: &str) -> Option<String> {
        for (intent, keywords) in &self.patterns {
            for keyword in keywords {
                if input.contains(keyword) {
                    return Some(intent.to_string());
                }
            }
        }
        None
    }
};