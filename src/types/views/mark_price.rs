use serde::{Deserialize, Serialize};

use crate::types::primitives::BlockTimestamp;

/// Mark price or implied volatility update.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MarkPriceView {
    Price {
        price: String,
        timestamp: BlockTimestamp,
    },
    PriceIV {
        price: String,
        iv: String,
        timestamp: BlockTimestamp,
    },
}
