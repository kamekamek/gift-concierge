use crate::app::error::{ChatError, Result};
use crate::app::nlp::IntentClassifier;
use std::collections::HashMap;
use log::{info, debug};

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
        let patterns = [
            ("relationship", vec![
                "お祝いをくださった方との関係を教えていただけますか？",
                "贈り主の方はどのような関係の方でしょうか？",
            ]),
            ("gift_amount", vec![
                "お祝いの金額を教えていただけますか？",
                "いただいたお祝いの金額はおいくらでしょうか？",
            ]),
            ("gift_type", vec![
                "複数の方へのお返しの場合、全員同じものにしますか？",
                "個別に選ばれますか？それとも全員同じものにされますか？",
            ]),
        ];

        for (key, messages) in patterns {
            self.response_patterns.insert(
                key.to_string(),
                messages.into_iter().map(String::from).collect()
            );
        }
    }

    pub fn process_input(&self, input: &str) -> Result<String> {
        debug!("Processing input: {}", input);
        
        if !self.validate_input(input) {
            return Err(ChatError::InvalidInput("入力が無効です".to_string()));
        }

        let intent = self.intent_classifier.classify(input)
            .ok_or_else(|| ChatError::InvalidInput("意図を理解できませんでした".to_string()))?;

        let response = self.get_response(&intent)
            .ok_or_else(|| ChatError::InternalError("適切な応答が見つかりません".to_string()))?;

        info!("Generated response for intent {}: {}", intent, response);
        Ok(response)
    }

    fn get_response(&self, intent: &str) -> Option<String> {
        self.response_patterns.get(intent)
            .and_then(|responses| responses.first())
            .cloned()
    }

    fn validate_input(&self, input: &str) -> bool {
        !input.trim().is_empty() && input.chars().count() <= 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validation() {
        let handler = ConversationHandler::new();
        assert!(handler.validate_input("正常な入力です"));
        assert!(!handler.validate_input(""));
        assert!(!handler.validate_input("   "));
    }

    #[test]
    fn test_process_input() {
        let handler = ConversationHandler::new();
        let result = handler.process_input("上司からのお祝い");
        assert!(result.is_ok());
    }
}