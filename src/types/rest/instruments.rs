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

pub type GetInstruments = PaginationData;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsResponse {
    pub instruments: InstrumentsSetView,
    pub instrument_stats: InstrumentStatsMapView,
    pub funding_rates: FundingRateByInstrumentClientView,
    pub daily_changes: HashMap<InstrumentId, InstrumentDailyPriceChange>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDailyPriceChange {
    pub price_change_quote: String,
    pub actual_available_data_time_range_ms: BlockTimestamp,
}
