use std::collections::HashMap;

pub struct IntentClassifier {
    patterns: HashMap<String, Vec<String>>,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert(
            "relationship".to_string(),
            vec!["上司".to_string(), "先輩".to_string(), "友人".to_string(), "親戚".to_string()]
        );
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