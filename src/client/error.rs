#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to send request or receive response: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Failed to (de)serialize response: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("WS error: {0}")]
    Ws(#[from] tungstenite::Error),
}
