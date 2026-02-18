use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AssetId, AssetTicker, HaircutTick, PriceTick, ScenarioChange, SCENARIO_COUNT,
};

pub type ScenarioGrid = [Scenario; SCENARIO_COUNT];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct Scenario {
    pub price: ScenarioChange,
    pub vol: ScenarioChange,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AssetRow {
    pub id: AssetId,
    pub ticker: AssetTicker,
    pub haircut: HaircutTick,
    pub mark_price: PriceTick,
    pub scenario_grid: ScenarioGrid,
    pub initial_scenario_grid: ScenarioGrid,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssetRowData {
    pub ticker: AssetTicker,
    pub haircut: HaircutTick,
    pub mark_price: PriceTick,
    pub scenario_grid: ScenarioGrid,
    pub initial_scenario_grid: ScenarioGrid,
}
