use serde::{Deserialize, Serialize};

use crate::types::primitives::{BlockTimestamp, CandleTimeframe, InstrumentId};

/// Candle data for WebSocket streaming - stores human-readable f32 values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleView {
    pub instrument_id: InstrumentId,
    pub interval: CandleTimeframe,
    slot: u64,
    pub timestamp: BlockTimestamp,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
    pub is_closed: bool,
}
