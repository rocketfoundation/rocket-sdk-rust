use serde::{Deserialize, Serialize};

use crate::types::primitives::AssetId;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WithdrawData {
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// String containing a decimal number
    pub amount: String,
    /// Target address on the external chain in hex format
    pub to: String,
}
