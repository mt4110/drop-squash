use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("file does not exist: {0}")]
    FileNotFound(PathBuf),
    #[error("file did not become stable before timeout: {0}")]
    FileStabilityTimeout(PathBuf),
    #[error("operation was cancelled")]
    Cancelled,
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("unsupported media: {0}")]
    UnsupportedMedia(String),
    #[error("encoder failed: {0}")]
    Encoder(String),
    #[error("history error: {0}")]
    History(String),
    #[error("license error: {0}")]
    License(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
