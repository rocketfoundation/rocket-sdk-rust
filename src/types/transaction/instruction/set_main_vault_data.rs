use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMainVaultData {
    pub main_vault_address: Option<AccountAddress>,
    /// Percent value between 0.0 and 100.0
    pub main_vault_adl_risk_limit_percent: Option<String>,
}
