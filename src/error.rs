use thiserror::Error;

#[derive(Error, Debug)]
pub enum FreesoundError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("API error: {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, FreesoundError>;
