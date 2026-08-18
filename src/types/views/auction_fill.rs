use serde::{Deserialize, Serialize};

use crate::types::primitives::BlockTimestamp;

/// Full fill view with price, used by the REST recent-trades endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionFillView {
    /// Fill price as a decimal string.
    pub price: String,
    /// Fill size as a decimal string.
    pub size: String,
    /// Timestamp in milliseconds.
    pub timestamp: BlockTimestamp,
}

/// Compact fill entry without price, used in the stream where all fills
/// in a single auction batch share the same price.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionFillEntry {
    /// Fill size as a decimal string.
    pub size: String,
    /// Timestamp in milliseconds.
    pub timestamp: BlockTimestamp,
}
