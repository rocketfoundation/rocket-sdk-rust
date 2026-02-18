use serde::{Deserialize, Serialize};

use crate::types::primitives::{AssetRowData, AssetTicker, ScenarioGrid};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListAssetsData {
    pub assets: Vec<AssetRowData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssetScenariosData {
    pub ticker: AssetTicker,
    pub scenario_grid: Option<ScenarioGrid>,
    pub initial_scenario_grid: Option<ScenarioGrid>,
}
