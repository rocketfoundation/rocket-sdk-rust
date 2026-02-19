use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, AssetId},
    views::collateral::CollateralView,
};

/// Request params to fetch collateral amounts for an account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCollateral {
    /// Account address whose collateral is queried.
    pub account: AccountAddress,
    /// Optional asset filter for collateral.
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
}

/// Response containing collateral mapping.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCollateralsResponse {
    /// Map of asset IDs to collateral amounts.
    pub collaterals: CollateralView,
}
