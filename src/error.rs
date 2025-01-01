use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("データベースエラー: {0}")]
    Database(#[from] sqlx::Error),

    #[error("API呼び出しエラー: {0}")]
    Api(#[from] reqwest::Error),

    #[error("環境変数エラー: {0}")]
    Env(#[from] std::env::VarError),

    #[error("入力値が不正です: {0}")]
    Validation(String),

    #[error("認証エラー: {0}")]
    Auth(String),

    #[error("内部エラー: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("データベースエラーが発生しました: {}", e),
            ),
            AppError::Api(ref e) => (
                StatusCode::BAD_GATEWAY,
                format!("外部APIとの通信でエラーが発生しました: {}", e),
            ),
            AppError::Env(ref e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("環境設定エラーが発生しました: {}", e),
            ),
            AppError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Auth(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Internal(ref msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("内部エラーが発生しました: {}", msg),
            ),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>; 