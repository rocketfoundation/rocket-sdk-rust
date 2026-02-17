use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, AssetId},
    views::collateral::CollateralView,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetCollateral {
    pub account: AccountAddress,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetCollateralsResponse {
    pub collaterals: CollateralView,
}
