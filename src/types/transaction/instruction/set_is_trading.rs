use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentId;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SetIsTradingData {
    pub instrument_id: InstrumentId,
    pub is_trading: bool,
}
