use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentId;

/// Payload for delisting an instrument.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelistInstrumentData {
    /// Identifier of the instrument to delist.
    pub instrument_id: InstrumentId,
}
