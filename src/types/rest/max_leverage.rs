use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, InstrumentId};

/// Request params to retrieve the maximum leverage setting for an account and instrument.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLeverage {
    /// Account address to query.
    pub account: AccountAddress,
    /// Instrument identifier.
    pub instrument_id: InstrumentId,
}

/// Response containing the max leverage setting.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLeverageResponse {
    /// Maximum leverage setting value.
    pub max_leverage_setting: u64,
}
