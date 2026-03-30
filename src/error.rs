pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid authorization header: {0}")]
    InvalidHeader(String),

    #[error("invalid proxy configuration: {0}")]
    InvalidProxy(#[source] reqwest::Error),

    #[error("invalid multipart payload: {0}")]
    InvalidMultipart(String),

    #[error("API error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("rate limited after {attempts} attempts")]
    RateLimited {
        attempts: u32,
        retry_after: Option<u64>,
        body: String,
    },

    #[error("retry limit exhausted for HTTP {status} after {attempts} attempts")]
    RetryExhausted {
        status: u16,
        attempts: u32,
        body: String,
    },
}
