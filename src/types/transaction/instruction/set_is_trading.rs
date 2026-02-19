use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentId;

/// Payload to toggle trading status of an instrument.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SetIsTradingData {
    /// Identifier of the instrument.
    pub instrument_id: InstrumentId,
    /// New trading enabled/disabled state.
    pub is_trading: bool,
}
