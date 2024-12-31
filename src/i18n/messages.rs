use std::collections::HashMap;
use serde_json::Value;
use std::fs;

pub struct I18nMessages {
    current_locale: String,
    messages: HashMap<String, Value>,
}

impl I18nMessages {
    pub fn new(locale: &str) -> Result<Self, std::io::Error> {
        let file_path = format!("src/i18n/{}.json", locale);
        let content = fs::read_to_string(file_path)?;
        let messages: HashMap<String, Value> = serde_json::from_str(&content)?;
        
        Ok(Self {
            current_locale: locale.to_string(),
            messages,
        })
    }

    pub fn get_message(&self, key: &str) -> Option<&str> {
        self.messages.get(key)
            .and_then(|v| v.as_str())
    }

    pub fn change_locale(&mut self, locale: &str) -> Result<(), std::io::Error> {
        let file_path = format!("src/i18n/{}.json", locale);
        let content = fs::read_to_string(file_path)?;
        let messages: HashMap<String, Value> = serde_json::from_str(&content)?;
        
        self.current_locale = locale.to_string();
        self.messages = messages;
        Ok(())
    }
} 