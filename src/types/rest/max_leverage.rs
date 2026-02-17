use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, InstrumentId};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLeverage {
    pub account: AccountAddress,
    pub instrument_id: InstrumentId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLeverageResponse {
    pub max_leverage_setting: u64,
}
