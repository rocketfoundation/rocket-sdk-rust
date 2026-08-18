use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentId;

/// Query parameters for indexer instrument 24h details.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentDetails {
    /// Instrument ID to fetch details for. When omitted, returns all indexed
    /// instruments (optionally narrowed by the filters below).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<InstrumentId>,
    /// Optional contract type filter (comma-separated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    /// Optional expiry filter (e.g. `28MAR25`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Optional underlying asset ticker filter (e.g. `BTC`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
}

/// Live 24h volume and price change for the requested instrument(s).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentDetailsResponse {
    pub instruments: Vec<InstrumentDetailsResponseItem>,
}

/// A single instrument details row.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDetailsResponseItem {
    pub instrument_id: InstrumentId,
    pub volume24hr: String,
    pub quote_volume: String,
    pub trade_count: u64,
    /// Quote price change over the 24h window (decimal string).
    pub change24hr: String,
}
