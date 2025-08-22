use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhoopError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to serialize/deserialize data: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Resource not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl WhoopError {
    /// Maps HTTP status codes to our error types.
    /// Helps us handle API errors consistently.
    pub fn from_status(status: reqwest::StatusCode, message: Option<String>) -> Self {
        let msg = message.unwrap_or_else(|| status.to_string());
        match status.as_u16() {
            400 => Self::BadRequest(msg),
            401 => Self::AuthenticationError(msg),
            404 => Self::NotFound,
            429 => Self::RateLimitExceeded,
            500..=599 => Self::ServerError(msg),
            _ => Self::Unknown(msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, WhoopError>;
