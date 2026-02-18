use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, InstrumentId};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageData {
    pub to: AccountAddress,
    pub instrument_id: InstrumentId,
    pub leverage: u64,
}
