use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, InstrumentId};

/// Payload for adjusting leverage on an account.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageData {
    /// Account address receiving the leverage change.
    pub to: AccountAddress,
    /// Instrument affected by the leverage change.
    pub instrument_id: InstrumentId,
    /// Desired leverage value.
    pub leverage: u64,
}
