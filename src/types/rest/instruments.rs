use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{
    primitives::{BlockTimestamp, InstrumentId},
    rest::pagination::PaginationData,
    views::{
        funding_rate::FundingRateByInstrumentClientView, instrument::InstrumentsSetView,
        instrument_stats::InstrumentStatsMapView,
    },
};

/// Request parameters for getting list of available instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstruments {
    /// Pagination params.
    #[serde(flatten)]
    pub pagination_data: PaginationData,
}

/// Response containing instrument metadata and related statistics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsResponse {
    /// Set of instruments returned.
    pub instruments: InstrumentsSetView,
    /// Statistics per instrument.
    pub instrument_stats: InstrumentStatsMapView,
    /// Current funding rates by instrument.
    pub funding_rates: FundingRateByInstrumentClientView,
    /// Daily price change per instrument.
    pub daily_changes: HashMap<InstrumentId, InstrumentDailyPriceChange>,
}

/// Daily price change data for an instrument.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDailyPriceChange {
    /// Quote-denominated price change over the day.
    pub price_change_quote: String,
    /// Actual time range (ms) of available data used for the calculation.
    pub actual_available_data_time_range_ms: BlockTimestamp,
}
