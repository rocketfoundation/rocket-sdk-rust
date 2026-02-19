use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentRowData;

/// Payload for listing instruments.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListInstrumentsData {
    /// Collection of instrument row data entries.
    pub instruments: Vec<InstrumentRowData>,
}
