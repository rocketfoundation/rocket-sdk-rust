use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AccountAddress, BlockTimestamp, GlobalOrderId, InstrumentId, OrderSide,
};

/// Details of an open order.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrderView {
    /// Identifier of the order.
    pub order_id: GlobalOrderId,
    /// Address of the account who placed the order.
    pub trader: AccountAddress,
    /// Type of the order (limit or market).
    pub order_type: OrderType,
    /// Optional trigger type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    /// Instrument the order is for.
    pub instrument_id: InstrumentId,
    /// Order price as a decimal string.
    pub price: String,
    /// Order quantity as a decimal string.
    pub quantity: String,
    /// Already filled quantity as a decimal string.
    pub filled_quantity: String,
    /// Side of the order (buy or sell).
    pub side: OrderSide,
    /// Timestamp when the order was created.
    pub timestamp: BlockTimestamp,
    /// Optional trigger price as a decimal string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
}

/// Type of order indicating execution behavior.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
}

/// Kind of trigger applied to an order.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum TriggerType {
    StopLoss,
    TakeProfit,
}
