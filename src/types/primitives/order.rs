use serde::{Deserialize, Serialize};

/// Flag specifying the order side.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}
