use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AccountAddress, AssetId, BlockTimestamp, GlobalOrderId, InstrumentId,
};

/// Query parameters for indexer non-fill order history.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistory {
    /// Account whose order history is requested.
    pub account: AccountAddress,
    /// Start timestamp in milliseconds (optional, defaults to 24h ago).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<BlockTimestamp>,
    /// End timestamp in milliseconds (optional, defaults to now).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<BlockTimestamp>,
    /// Maximum number of order events to return (optional, defaults to 500, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

/// Non-fill order events for the requested account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryResponse {
    pub order_history: Vec<OrderHistoryResponseItem>,
}

/// A single non-fill order event from the indexer.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResponseItem {
    pub timestamp: BlockTimestamp,
    pub round: u64,
    pub tx_index: u32,
    pub event_index: u16,
    pub account: AccountAddress,
    pub instrument: InstrumentId,
    pub order_id: GlobalOrderId,
    pub event_type: String,
    pub price: String,
    pub size: String,
    pub settlement_asset: AssetId,
    pub pnl: String,
    pub fee_amount: String,
    pub fee_rate: String,
    pub is_passive: bool,
    pub is_liquidation: bool,
    pub is_adl: bool,
}
