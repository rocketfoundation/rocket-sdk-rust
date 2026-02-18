use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelistInstrumentData {
    pub instrument_id: InstrumentId,
}
