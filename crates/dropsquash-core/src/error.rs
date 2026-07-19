use std::path::PathBuf;

pub const NOT_SMALLER_MESSAGE: &str =
    "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip.";
pub const CONVERSION_FAILED_MESSAGE: &str =
    "This recording could not be converted, so DropSquash kept the original and did not count the attempt.";
pub const LICENSE_NETWORK_MESSAGE: &str =
    "DropSquash could not reach the license server. Check your connection and try again. Your existing license on this Mac stayed unchanged.";

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

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Encoder(message) => normalize_encoder_message(message),
            Self::License(message) if message.contains("License server is unreachable.") => {
                LICENSE_NETWORK_MESSAGE.to_string()
            }
            _ => self.to_string(),
        }
    }
}

pub fn normalize_encoder_message(message: &str) -> String {
    let message = strip_error_prefixes(message);
    if is_not_smaller_error(message) {
        return NOT_SMALLER_MESSAGE.to_string();
    }
    if message.contains("The operation could not be completed") {
        return CONVERSION_FAILED_MESSAGE.to_string();
    }
    message.to_string()
}

fn is_not_smaller_error(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("output is not smaller")
        || lower.contains("not smaller than original")
        || lower.contains("could not be made smaller")
}

fn strip_error_prefixes(message: &str) -> &str {
    let message = message.trim();
    if let Some(stripped) = message.strip_prefix("Error: ") {
        return strip_error_prefixes(stripped);
    }
    if let Some(stripped) = message.strip_prefix("encoder failed: ") {
        return strip_error_prefixes(stripped);
    }
    message
}

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests;
