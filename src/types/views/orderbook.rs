use serde::{Deserialize, Serialize};

/// Order book level info.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LevelView {
    pub price: String,
    pub quantity: String,
}

/// Orderbook contents.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderbookView {
    pub bids: Vec<LevelView>,
    pub asks: Vec<LevelView>,
}
