use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, BlockTimestamp},
    views::vault_stats::VaultStatsSetView,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultStats {
    /// Comma-separated list of vault addresses (e.g., "0xABC...,0xDEF...")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault_addresses: Option<Vec<AccountAddress>>,
    /// Milliseconds, first second of the day UTC
    pub from: BlockTimestamp,
    /// Milliseconds, first second of the day UTC
    pub to: BlockTimestamp,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultStatsResponse {
    pub vault_stats: VaultStatsSetView,
}
