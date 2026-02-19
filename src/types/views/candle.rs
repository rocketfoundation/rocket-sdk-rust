use serde::{Deserialize, Serialize};

use crate::types::primitives::{BlockTimestamp, CandleTimeframe, InstrumentId};

/// Candle data for WebSocket streaming - stores human-readable f32 values.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleView {
    /// Instrument id.
    pub instrument_id: InstrumentId,
    /// Timeframe interval of the candle.
    pub interval: CandleTimeframe,
    /// Internal slot counter.
    pub slot: u64,
    /// Timestamp of the candle open.
    pub timestamp: BlockTimestamp,
    /// Opening price.
    pub open: f32,
    /// Highest price during interval.
    pub high: f32,
    /// Lowest price during interval.
    pub low: f32,
    /// Closing price.
    pub close: f32,
    /// Trading volume during the interval.
    pub volume: f32,
    /// Indicates whether the candle period has closed.
    pub is_closed: bool,
}
