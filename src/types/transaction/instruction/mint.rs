use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetId};

/// Payload for minting new tokens to an account.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MintData {
    /// Recipient account address.
    pub to: AccountAddress,
    /// Identifier of the asset to mint.
    pub asset_id: AssetId,
    /// Amount to mint as a decimal string.
    pub amount: String,
}
