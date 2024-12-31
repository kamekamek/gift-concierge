use std::env;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub ttl_seconds: u64,
    pub max_size: usize,
    pub cleanup_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub perplexity_api_key: String,
    pub perplexity_api_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationConfig {
    pub default_language: String,
    pub available_languages: Vec<String>,
    pub fallback_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: PathBuf,
    pub rotation_size: u64,
    pub max_files: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub environment: String,
    pub server_port: u16,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub api: ApiConfig,
    pub localization: LocalizationConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let config = Config {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Failed to parse SERVER_PORT")?,
            
            database: DatabaseConfig {
                host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("DB_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .context("Failed to parse DB_PORT")?,
                username: env::var("DB_USERNAME").context("DB_USERNAME not set")?,
                password: env::var("DB_PASSWORD").context("DB_PASSWORD not set")?,
                database_name: env::var("DB_NAME").context("DB_NAME not set")?,
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .context("Failed to parse DB_MAX_CONNECTIONS")?,
            },
            
            cache: CacheConfig {
                ttl_seconds: env::var("CACHE_TTL_SECONDS")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .context("Failed to parse CACHE_TTL_SECONDS")?,
                max_size: env::var("CACHE_MAX_SIZE")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()
                    .context("Failed to parse CACHE_MAX_SIZE")?,
                cleanup_interval: env::var("CACHE_CLEANUP_INTERVAL")
                    .unwrap_or_else(|_| "300".to_string())
                    .parse()
                    .context("Failed to parse CACHE_CLEANUP_INTERVAL")?,
            },
            
            api: ApiConfig {
                perplexity_api_key: env::var("PERPLEXITY_API_KEY").context("PERPLEXITY_API_KEY not set")?,
                perplexity_api_url: env::var("PERPLEXITY_API_URL")
                    .unwrap_or_else(|_| "https://api.perplexity.ai".to_string()),
                timeout_seconds: env::var("API_TIMEOUT_SECONDS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .context("Failed to parse API_TIMEOUT_SECONDS")?,
                max_retries: env::var("API_MAX_RETRIES")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()
                    .context("Failed to parse API_MAX_RETRIES")?,
            },
            
            localization: LocalizationConfig {
                default_language: env::var("DEFAULT_LANGUAGE")
                    .unwrap_or_else(|_| "ja".to_string()),
                available_languages: env::var("AVAILABLE_LANGUAGES")
                    .unwrap_or_else(|_| "ja,en".to_string())
                    .split(',')
                    .map(String::from)
                    .collect(),
                fallback_language: env::var("FALLBACK_LANGUAGE")
                    .unwrap_or_else(|_| "en".to_string()),
            },
            
            logging: LoggingConfig {
                level: env::var("LOG_LEVEL")
                    .unwrap_or_else(|_| "info".to_string()),
                file_path: PathBuf::from(
                    env::var("LOG_FILE_PATH")
                        .unwrap_or_else(|_| "logs/app.log".to_string())
                ),
                rotation_size: env::var("LOG_ROTATION_SIZE")
                    .unwrap_or_else(|_| "10485760".to_string()) // 10MB
                    .parse()
                    .context("Failed to parse LOG_ROTATION_SIZE")?,
                max_files: env::var("LOG_MAX_FILES")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .context("Failed to parse LOG_MAX_FILES")?,
            },
        };

        Ok(config)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let config_str = fs::read_to_string(path)
            .context("Failed to read config file")?;
        let config: Config = serde_json::from_str(&config_str)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn to_file(&self, path: &str) -> Result<()> {
        let config_str = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(path, config_str)
            .context("Failed to write config file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            environment: "test".to_string(),
            server_port: 8080,
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "test_user".to_string(),
                password: "test_pass".to_string(),
                database_name: "test_db".to_string(),
                max_connections: 10,
            },
            cache: CacheConfig {
                ttl_seconds: 3600,
                max_size: 1000,
                cleanup_interval: 300,
            },
            api: ApiConfig {
                perplexity_api_key: "test_key".to_string(),
                perplexity_api_url: "https://api.test.com".to_string(),
                timeout_seconds: 30,
                max_retries: 3,
            },
            localization: LocalizationConfig {
                default_language: "ja".to_string(),
                available_languages: vec!["ja".to_string(), "en".to_string()],
                fallback_language: "en".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: PathBuf::from("test.log"),
                rotation_size: 10485760,
                max_files: 5,
            },
        };

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // 設定をファイルに保存
        config.to_file(path).unwrap();

        // 設定をファイルから読み込み
        let loaded_config = Config::from_file(path).unwrap();

        // 設定が正しく保存・読み込みされたことを確認
        assert_eq!(config.environment, loaded_config.environment);
        assert_eq!(config.server_port, loaded_config.server_port);
        assert_eq!(config.database.host, loaded_config.database.host);
        assert_eq!(config.cache.ttl_seconds, loaded_config.cache.ttl_seconds);
        assert_eq!(config.api.perplexity_api_key, loaded_config.api.perplexity_api_key);
        assert_eq!(config.localization.default_language, loaded_config.localization.default_language);
        assert_eq!(config.logging.level, loaded_config.logging.level);
    }
} 