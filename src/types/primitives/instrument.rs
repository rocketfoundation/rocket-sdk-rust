use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AssetId, BlockTimestamp, InstrumentId, PriceScale, PriceTick, QuantityScale, QuantityTick,
    SCENARIO_COUNT,
};

/// Grid of possible profit and loss for each scenario.
pub type PnLGrid = [PnLTick; SCENARIO_COUNT];

/// Individual profit and loss tick value.
pub type PnLTick = i64;

/// Bitflags representing properties of an instrument.
pub type InstrumentFlags = u16;

/// Flag indicating a spot instrument.
pub const SPOT_INSTRUMENT: InstrumentFlags = 1 << 0;

/// Flag indicating a perpetual instrument.
pub const PERPETUAL_INSTRUMENT: InstrumentFlags = 1 << 1;

/// Flag indicating a future instrument.
pub const FUTURE_INSTRUMENT: InstrumentFlags = 1 << 2;

/// Flag indicating a call option instrument.
pub const CALL_OPTION_INSTRUMENT: InstrumentFlags = 1 << 3;

/// Flag indicating a put option instrument.
pub const PUT_OPTION_INSTRUMENT: InstrumentFlags = 1 << 4;

/// Instrument data.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct InstrumentRow {
    /// Unique identifier for the instrument.
    pub instrument_id: InstrumentId,
    /// ID of the underlying asset.
    pub underlying_asset_id: AssetId,
    /// ID of the settlement asset.
    pub settlement_asset_id: AssetId,
    /// Expiry timestamp in milliseconds.
    pub expiry: BlockTimestamp,
    /// Strike price value.
    pub strike: PriceTick,
    /// Scale factor for price values.
    pub price_scale: PriceScale,
    /// Scale factor for quantity values.
    pub quantity_scale: QuantityScale,
    /// Flags describing the instrument type.
    pub instrument_flags: InstrumentFlags,
    /// Latest mark price.
    pub mark_price: PriceTick,
    /// Trading enabled/disabled flag.
    pub is_trading: bool,
    /// Price of the last matched order.
    pub last_match_price: PriceTick,
    /// Volume of the last matched order.
    pub last_match_volume: QuantityTick,
    /// Profit and loss grid, used for risk measurement of existing orders.
    pub pnl_grid: PnLGrid,
    /// Initial profit and loss grid, used for risk measurement when placing new orders.
    pub initial_pnl_grid: PnLGrid,
}

/// Instrument data without id and latest match data.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstrumentRowData {
    /// ID of the underlying asset.
    pub underlying_asset_id: AssetId,
    /// ID of the settlement asset.
    pub settlement_asset_id: AssetId,
    /// Expiry timestamp in milliseconds.
    pub expiry: BlockTimestamp,
    /// Strike price value.
    pub strike: PriceTick,
    /// Scale factor for price values.
    pub price_scale: PriceScale,
    /// Scale factor for quantity values.
    pub quantity_scale: QuantityScale,
    /// Flags describing the instrument type.
    pub instrument_flags: InstrumentFlags,
    /// Trading enabled/disabled flag.
    pub is_trading: bool,
    /// Profit and loss grid, used for risk measurement of existing orders.
    pub pnl_grid: PnLGrid,
    /// Initial profit and loss grid, used for risk measurement when placing new orders.
    pub initial_pnl_grid: PnLGrid,
}
