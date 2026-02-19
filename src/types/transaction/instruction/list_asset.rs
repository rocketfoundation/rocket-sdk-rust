use serde::{Deserialize, Serialize};

use crate::types::primitives::{AssetRowData, AssetTicker, ScenarioGrid};

/// Payload for listing assets.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListAssetsData {
    /// Vector of asset row data entries.
    pub assets: Vec<AssetRowData>,
}

/// Data for updating scenario grids for a specific asset.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssetScenariosData {
    /// Ticker of the asset to update.
    pub ticker: AssetTicker,
    /// Optional new scenario grid.
    pub scenario_grid: Option<ScenarioGrid>,
    /// Optional new initial scenario grid.
    pub initial_scenario_grid: Option<ScenarioGrid>,
}
