use serde::{Deserialize, Serialize};

/// Lightweight quote snapshot for price feed subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteView {
    pub timestamp: u64,
    pub bid_price: String,
    pub bid_size: String,
    pub ask_price: String,
    pub ask_size: String,
}
