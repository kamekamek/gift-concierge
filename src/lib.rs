pub mod app {
    pub mod chat {
        pub mod chatbot;
        pub mod conversation_handler;
    }
    pub mod nlp {
        pub mod intent_classifier;
    }
    pub mod gift {
        pub mod recommendation;
    }
    pub mod database {
        pub mod user_record;
        pub mod gift_cache;
    }
}

pub mod config {
    pub mod config;
} 