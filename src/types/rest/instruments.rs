use serde::{Deserialize, Serialize};

use crate::types::{
    rest::pagination::PaginationData,
    views::{instrument::InstrumentsSetView, instrument_stats::InstrumentStatsMapView},
};

/// Request parameters for getting list of available instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstruments {
    /// Pagination params.
    #[serde(flatten)]
    pub pagination_data: PaginationData,
    /// Optional contract type filter (comma-separated). Accepted values
    /// (case-insensitive): `option`, `future`, `perp`, `spot`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    /// Optional expiry filter. Matches the instrument's formatted expiry (e.g. `28MAR25`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Optional underlying asset ticker filter (e.g. `BTC`). Case-insensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
}

/// Response containing instrument metadata and related statistics.
///
/// Node `/instruments` includes live `instrument_stats`. Indexer `/instruments`
/// returns only `instruments` (historical + current metadata).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsResponse {
    /// Set of instruments returned.
    pub instruments: InstrumentsSetView,
    /// Latest per-instrument market stats. Empty when omitted (indexer).
    #[serde(default)]
    pub instrument_stats: InstrumentStatsMapView,
}
