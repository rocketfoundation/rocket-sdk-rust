use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AssetId, AssetTicker, HaircutTick, PriceTick, ScenarioChange, SCENARIO_COUNT,
};

/// Fixed-size grid holding possible `Scenario` instances, used for risk measurements.
pub type ScenarioGrid = [Scenario; SCENARIO_COUNT];

/// Scenario used for risk estimation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct Scenario {
    /// Price change for the scenario.
    pub price: ScenarioChange,
    /// Volatility change for the scenario.
    pub vol: ScenarioChange,
}

/// Asset data.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AssetRow {
    /// Unique identifier for the asset.
    pub id: AssetId,
    /// Ticker symbol of the asset.
    pub ticker: AssetTicker,
    /// Haircut applied to the asset.
    pub haircut: HaircutTick,
    /// Latest mark price of the asset.
    pub mark_price: PriceTick,
    /// Grid of scenarios representing possible price and volatility changes,
    /// used for margin calculation for existing orders.
    pub scenario_grid: ScenarioGrid,
    /// Initial grid of scenarios representing possible price and volatility changes,
    /// used for margin calculation when placing new orders.
    pub initial_scenario_grid: ScenarioGrid,
}

/// Asset data without an id.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssetRowData {
    /// Ticker symbol of the asset.
    pub ticker: AssetTicker,
    /// Haircut applied to the asset.
    pub haircut: HaircutTick,
    /// Latest mark price of the asset.
    pub mark_price: PriceTick,
    /// Grid of scenarios representing possible price and volatility changes,
    /// used for margin calculation for existing orders.
    pub scenario_grid: ScenarioGrid,
    /// Initial grid of scenarios representing possible price and volatility changes,
    /// used for margin calculation when placing new orders.
    pub initial_scenario_grid: ScenarioGrid,
}
