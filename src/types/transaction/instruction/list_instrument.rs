use serde::{Deserialize, Serialize};

use crate::types::primitives::InstrumentRowData;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListInstrumentsData {
    pub instruments: Vec<InstrumentRowData>,
}
