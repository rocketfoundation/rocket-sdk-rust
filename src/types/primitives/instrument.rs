use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AssetId, BlockTimestamp, InstrumentId, PriceScale, PriceTick, QuantityScale, QuantityTick,
    SCENARIO_COUNT,
};

pub type PnLGrid = [PnLTick; SCENARIO_COUNT];
pub type PnLTick = i64;

pub type InstrumentFlags = u16;
pub const SPOT_INSTRUMENT: InstrumentFlags = 1 << 0;
pub const PERPETUAL_INSTRUMENT: InstrumentFlags = 1 << 1;
pub const FUTURE_INSTRUMENT: InstrumentFlags = 1 << 2;
pub const CALL_OPTION_INSTRUMENT: InstrumentFlags = 1 << 3;
pub const PUT_OPTION_INSTRUMENT: InstrumentFlags = 1 << 4;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct InstrumentRow {
    pub instrument_id: InstrumentId,
    pub underlying_asset_id: AssetId,
    pub settlement_asset_id: AssetId,
    pub expiry: BlockTimestamp,
    pub strike: PriceTick,
    pub price_scale: PriceScale,
    pub quantity_scale: QuantityScale,
    pub instrument_flags: InstrumentFlags,
    pub mark_price: PriceTick,
    pub is_trading: bool,
    pub last_match_price: PriceTick,
    pub last_match_volume: QuantityTick,
    pub pnl_grid: PnLGrid,
    pub initial_pnl_grid: PnLGrid,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstrumentRowData {
    pub underlying_asset_id: AssetId,
    pub settlement_asset_id: AssetId,
    pub expiry: BlockTimestamp,
    pub strike: PriceTick,
    pub price_scale: PriceScale,
    pub quantity_scale: QuantityScale,
    pub instrument_flags: InstrumentFlags,
    pub is_trading: bool,
    pub pnl_grid: PnLGrid,
    pub initial_pnl_grid: PnLGrid,
}
