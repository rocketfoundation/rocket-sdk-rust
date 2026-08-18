use serde::{Deserialize, Serialize};

use crate::types::{primitives::AccountAddress, views::vault_depositors::VaultDepositorsView};

/// Request params for vault depositors on the node.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultDepositors {
    /// Vault address.
    pub vault: AccountAddress,
}

/// Non-zero depositors for the requested vault.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultDepositorsResponse {
    pub vault_depositors: VaultDepositorsView,
}
