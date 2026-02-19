use serde::{Deserialize, Serialize};

use crate::types::primitives::{BlockTimestamp, CandleTimeframe, InstrumentId};

/// Query parameters for historical candles.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCandles {
    /// Instrument ID to fetch candles for.
    pub instrument_id: InstrumentId,
    /// Candle timeframe (1m, 5m, 15m, 30m, 1h, 4h, 1d).
    pub interval: CandleTimeframe,
    /// Start timestamp in milliseconds (optional, defaults to 24h ago).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<BlockTimestamp>,
    /// End timestamp in milliseconds (optional, defaults to now).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<BlockTimestamp>,
    /// Maximum number of candles to return (optional, defaults to 500, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

/// Response containing historical candles.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCandlesResponse {
    /// List of OHLCV candles.
    pub candles: Vec<CandleResponseItem>,
}

/// A single candle in the response (human-readable float values).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CandleResponseItem {
    /// Slot number (timestamp / interval_duration).
    pub slot: u64,
    /// Start timestamp of the candle in milliseconds.
    pub timestamp: BlockTimestamp,
    /// Opening price.
    pub open: f32,
    /// Highest price.
    pub high: f32,
    /// Lowest price.
    pub low: f32,
    /// Closing price.
    pub close: f32,
    /// Total volume traded.
    pub volume: f32,
}
