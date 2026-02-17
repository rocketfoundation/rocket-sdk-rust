use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::primitives::InstrumentId;

pub type InstrumentStatsMapView = HashMap<InstrumentId, InstrumentStatsView>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstrumentStatsView {
    #[serde(rename = "fundingRate1H")]
    pub funding_rate_1h: f64,
    #[serde(rename = "volume24H")]
    pub volume_24h: f64,
    #[serde(rename = "openInterest")]
    pub open_interest: f64,
}
