use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetId};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MintData {
    pub to: AccountAddress,
    pub asset_id: AssetId,
    /// String containing a decimal number
    pub amount: String,
}
