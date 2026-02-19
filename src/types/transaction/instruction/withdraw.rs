use serde::{Deserialize, Serialize};

use crate::types::primitives::AssetId;

/// Payload for withdrawing assets to an external chain.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WithdrawData {
    /// Identifier of the asset to withdraw.
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Amount to withdraw as a decimal string.
    pub amount: String,
    /// Target external chain address in hex format.
    pub to: String,
}
