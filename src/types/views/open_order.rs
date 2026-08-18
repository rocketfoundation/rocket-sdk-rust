use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, BlockTimestamp, GlobalOrderId, InstrumentId, OrderSide},
    views::instrument_type::AggregatedInstrumentType,
};

/// Details of an open order.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrderView {
    /// Identifier of the order.
    pub order_id: GlobalOrderId,
    /// Address of the account who placed the order.
    pub trader: AccountAddress,
    /// Type of the order (limit, market, or twap).
    pub order_type: OrderType,
    /// Optional trigger type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    /// Instrument the order is for.
    pub instrument_id: InstrumentId,
    /// Aggregated instrument type (spot, perpetuals, futures, options).
    #[serde(default)]
    pub instrument_type: AggregatedInstrumentType,
    /// Instrument ticker.
    #[serde(default)]
    pub ticker: String,
    /// Underlying asset ticker.
    #[serde(default)]
    pub underlying_asset: String,
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
    /// TWAP interval in milliseconds, when the order is a TWAP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twap_interval: Option<u64>,
    /// Original TWAP quantity as a decimal string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twap_original_quantity: Option<String>,
    /// Filled TWAP quantity as a decimal string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twap_filled_quantity: Option<String>,
    /// TWAP step frequency in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twap_frequency: Option<u64>,
    /// Whether TWAP step timing is randomized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twap_randomize: Option<bool>,
}

/// Type of order indicating execution behavior.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    Twap,
}

/// Kind of trigger applied to an order.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum TriggerType {
    StopLoss,
    TakeProfit,
}
