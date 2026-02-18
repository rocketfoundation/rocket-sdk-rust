use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetId};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultData {
    pub deposit_asset: AssetId,
    /// String containing a decimal number
    pub initial_deposit: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VaultDepositData {
    pub vault: AccountAddress,
    /// String containing a decimal number
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VaultWithdrawData {
    pub vault: AccountAddress,
    /// String containing a decimal number
    pub shares: String,
}
