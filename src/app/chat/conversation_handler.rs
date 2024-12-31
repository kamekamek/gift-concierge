use crate::app::error::{ChatError, Result};
use crate::app::nlp::IntentClassifier;
use std::collections::HashMap;

pub struct ConversationHandler {
    intent_classifier: IntentClassifier,
    response_patterns: HashMap<String, Vec<String>>,
}

impl ConversationHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            intent_classifier: IntentClassifier::new(),
            response_patterns: HashMap::new(),
        };
        handler.initialize_patterns();
        handler
    }

    fn initialize_patterns(&mut self) {
        self.response_patterns.insert(
            "relationship".to_string(),
            vec![
                "お祝いをくださった方との関係を教えていただけますか？",
                "贈り主の方はどのような関係の方でしょうか？",
            ].into_iter().map(String::from).collect()
        );
    }

    pub fn process_input(&self, input: &str) -> Result<String> {
        if !self.validate_input(input) {
            return Err(ChatError::InvalidInput("入力が無効です".to_string()));
        }

        let intent = self.intent_classifier.classify(input);
        Ok(self.get_response(intent))
    }

    fn validate_input(&self, input: &str) -> bool {
        !input.trim().is_empty() && input.chars().count() <= 1000
    }

    // 他のメソッドも同様に実装...
}