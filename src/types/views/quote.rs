use serde::{Deserialize, Serialize};

/// Lightweight quote snapshot for price feed subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteView {
    /// Unix timestamp of the quote update milliseconds.
    pub timestamp: u64,
    /// Best bid price as a decimal string.
    pub bid_price: String,
    /// Size available at the bid price as a decimal string.
    pub bid_size: String,
    /// Best ask price as a decimal string.
    pub ask_price: String,
    /// Size available at the ask price as a decimal string.
    pub ask_size: String,
}
