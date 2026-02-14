#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("LLM API error ({status}): {body}")]
    ApiError { status: u16, body: String },
    #[error("Invalid header: {0}")]
    InvalidHeader(String),
    #[error("Body must be a JSON object")]
    InvalidBody,
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Expected Assistant message, got: {0}")]
    UnexpectedMessage(String),
    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;
