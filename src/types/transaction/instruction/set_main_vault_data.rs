use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Payload for configuring main vault parameters.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMainVaultData {
    /// Optional new address of the main vault.
    pub main_vault_address: Option<AccountAddress>,
    /// Optional ADL risk limit percent as a decimal string between 0.0 and 100.0.
    pub main_vault_adl_risk_limit_percent: Option<String>,
}
