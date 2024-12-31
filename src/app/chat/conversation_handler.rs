use std::collections::HashMap;
use crate::app::nlp::intent_classifier::{IntentClassifier, Intent};

pub struct ConversationHandler {
    intent_classifier: IntentClassifier,
    response_patterns: HashMap<Intent, Vec<String>>,
    etiquette_responses: HashMap<String, String>,
}

impl ConversationHandler {
    pub fn new() -> Self {
        let mut handler = ConversationHandler {
            intent_classifier: IntentClassifier::new(),
            response_patterns: HashMap::new(),
            etiquette_responses: HashMap::new(),
        };
        handler.initialize_patterns();
        handler
    }

    fn initialize_patterns(&mut self) {
        self.response_patterns.insert(
            Intent::AskRelationship,
            vec![
                "お祝いをくださった方との関係を教えていただけますか？",
                "贈り主の方はどのような関係の方でしょうか？",
            ].into_iter().map(String::from).collect()
        );

        self.etiquette_responses.insert(
            "timing".to_string(),
            "お返しは一般的に1ヶ月以内が望ましいとされています。".to_string(),
        );
    }

    pub fn process_input(&self, input: &str) -> String {
        let intent = self.intent_classifier.classify(input);
        match intent {
            Intent::AskEtiquette => self.handle_etiquette_question(input),
            _ => self.get_response(intent),
        }
    }

    fn handle_etiquette_question(&self, input: &str) -> String {
        for (key, response) in &self.etiquette_responses {
            if input.contains(key) {
                return response.clone();
            }
        }
        "申し訳ありません。その質問にはお答えできません。".to_string()
    }

    fn get_response(&self, intent: Intent) -> String {
        if let Some(patterns) = self.response_patterns.get(&intent) {
            if let Some(response) = patterns.first() {
                return response.clone();
            }
        }
        "ご質問の意図を理解できませんでした。".to_string()
    }

    pub fn validate_input(&self, input: &str) -> bool {
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
}