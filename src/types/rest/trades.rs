use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AccountAddress, AssetId, BlockTimestamp, GlobalOrderId, InstrumentId,
};

/// Query parameters for indexer fill history.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTrades {
    /// Account address. Required if `instrument` is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountAddress>,
    /// Instrument id. Required if `account` is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentId>,
    /// Start timestamp in milliseconds (optional, defaults to 24h ago).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<BlockTimestamp>,
    /// End timestamp in milliseconds (optional, defaults to now).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<BlockTimestamp>,
    /// Maximum number of trades to return (optional, defaults to 500, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Alternative to `startTime`/`endTime`: number of latest trades per page (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    /// Opaque cursor returned by the previous `count` mode response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Fill events for the requested account and/or instrument.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesResponse {
    pub trades: Vec<TradeResponseItem>,
    /// Number of trades requested per page when `count` pagination mode is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    /// Cursor for the next page when `count` pagination mode is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// A single fill from the indexer trades endpoint.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponseItem {
    pub timestamp: BlockTimestamp,
    pub round: u64,
    pub tx_index: u32,
    pub event_index: u16,
    pub account: AccountAddress,
    pub instrument: InstrumentId,
    /// Canonical instrument ticker from the indexed `instruments` table (empty if unknown).
    pub ticker: String,
    pub order_id: GlobalOrderId,
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
