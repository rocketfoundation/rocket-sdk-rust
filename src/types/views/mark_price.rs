use serde::{Deserialize, Serialize};

use crate::types::primitives::BlockTimestamp;

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
