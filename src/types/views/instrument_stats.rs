use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::primitives::InstrumentId;

/// Mapping of instrument IDs to their statistics view.
pub type InstrumentStatsMapView = HashMap<InstrumentId, InstrumentStatsView>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstrumentStatsView {
    /// One-hour funding rate.
    #[serde(rename = "fundingRate1H")]
    pub funding_rate_1h: f64,
    /// 24-hour trading volume.
    #[serde(rename = "volume24H")]
    pub volume_24h: f64,
    /// Open interest value.
    #[serde(rename = "openInterest")]
    pub open_interest: f64,
}
