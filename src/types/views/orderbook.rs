use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LevelView {
    pub price: String,
    pub quantity: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderbookView {
    pub bids: Vec<LevelView>,
    pub asks: Vec<LevelView>,
}
