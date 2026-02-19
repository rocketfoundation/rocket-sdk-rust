use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetId};

/// Payload to create a new vault.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultData {
    /// Asset identifier.
    pub deposit_asset: AssetId,
    /// Initial deposit amount as a decimal string.
    pub initial_deposit: String,
}

/// Payload for depositing into a vault.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VaultDepositData {
    /// Address of the vault receiving the deposit.
    pub vault: AccountAddress,
    /// Amount to deposit as a decimal string.
    pub amount: String,
}

/// Payload for withdrawing from a vault.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VaultWithdrawData {
    /// Address of the vault to withdraw from.
    pub vault: AccountAddress,
    /// Number of shares to withdraw as a decimal string.
    pub shares: String,
}
