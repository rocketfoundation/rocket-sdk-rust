use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, BlockTimestamp},
    views::vault_stats::VaultStatsSetView,
};

/// Request params to get vault stats.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultStats {
    /// Comma-separated list of vault addresses (e.g., "0xABC...,0xDEF...").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault_addresses: Option<Vec<AccountAddress>>,
    /// Timestamp in milliseconds, first second of the day UTC.
    pub from: BlockTimestamp,
    /// Timestamp in milliseconds, first second of the day UTC.
    pub to: BlockTimestamp,
}

/// Response for vault stats.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultStatsResponse {
    /// Collection of stats for each requested vault.
    /// If `vault_address` in [GetVaultStats] is left empty, contains stats for all existing vaults.
    pub vault_stats: VaultStatsSetView,
}
