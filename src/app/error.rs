use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("入力が無効です: {0}")]
    InvalidInput(String),
    
    #[error("APIエラー: {0}")]
    ApiError(String),
    
    #[error("内部エラー: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, ChatError>; 