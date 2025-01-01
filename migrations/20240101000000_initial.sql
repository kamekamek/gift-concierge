-- Create chat_history table
CREATE TABLE IF NOT EXISTS chat_history (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    response TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_chat_history_user_id ON chat_history(user_id);
CREATE INDEX idx_chat_history_created_at ON chat_history(created_at);

-- Create gift_recommendations table
CREATE TABLE IF NOT EXISTS gift_recommendations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT,
    category VARCHAR(100) NOT NULL,
    rating FLOAT NOT NULL,
    source VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_gift_recommendations_price ON gift_recommendations(price);
CREATE INDEX idx_gift_recommendations_category ON gift_recommendations(category);
CREATE INDEX idx_gift_recommendations_rating ON gift_recommendations(rating);

-- Create user_preferences table
CREATE TABLE IF NOT EXISTS user_preferences (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) UNIQUE NOT NULL,
    budget_min INTEGER,
    budget_max INTEGER,
    preferred_categories TEXT[] NOT NULL DEFAULT '{}',
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_user_preferences_user_id ON user_preferences(user_id); 