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

/// Compact ticker payload. Field names are deliberately short to minimize
/// wire size for high-frequency ticker streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerView {
    /// Timestamp (ms).
    #[serde(rename = "t")]
    pub timestamp: u64,
    /// Ask size.
    #[serde(rename = "A")]
    pub ask_size: Option<String>,
    /// Ask price.
    #[serde(rename = "a")]
    pub ask_price: Option<String>,
    /// Bid size.
    #[serde(rename = "B")]
    pub bid_size: Option<String>,
    /// Bid price.
    #[serde(rename = "b")]
    pub bid_price: Option<String>,
    /// Instrument mark price.
    #[serde(rename = "I")]
    pub mark_price: Option<String>,
    /// Mid price ((bid + ask) / 2).
    #[serde(rename = "M")]
    pub mid_price: Option<String>,
    /// Implied volatility (options only).
    #[serde(rename = "V")]
    pub iv: Option<String>,
    /// Black-Scholes delta (options only).
    #[serde(rename = "D")]
    pub delta: Option<String>,
    /// Black-Scholes gamma (options only).
    #[serde(rename = "G")]
    pub gamma: Option<String>,
    /// Black-Scholes theta (options only).
    #[serde(rename = "H")]
    pub theta: Option<String>,
    /// Black-Scholes vega (options only).
    #[serde(rename = "e")]
    pub vega: Option<String>,
    /// Black-Scholes rho (options only).
    #[serde(rename = "R")]
    pub rho: Option<String>,
}
