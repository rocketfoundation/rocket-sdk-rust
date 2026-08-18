use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::primitives::InstrumentId;

/// Mapping of instrument IDs to their statistics view.
pub type InstrumentStatsMapView = HashMap<InstrumentId, InstrumentStatsView>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentStatsView {
    /// Open interest value.
    pub open_interest: f64,
    /// Forecast 1h funding rate as a decimal string, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecast_funding_rate: Option<String>,
    /// Premium index as a decimal string, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_index: Option<String>,
    /// 24-hour trading volume (base).
    pub volume_24h: String,
    /// 24-hour trading volume (quote).
    pub quote_volume_24h: String,
    /// 24-hour price change as a decimal string.
    pub price_change_24h: String,
}
