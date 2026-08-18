use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentId;

/// Payload for setting an instrument's maximum funding rate.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SetMaxFundingRateData {
    /// Instrument whose max funding rate is being set.
    pub instrument_id: InstrumentId,
    /// Maximum funding rate in native ticks.
    pub max_funding_rate: i64,
}
