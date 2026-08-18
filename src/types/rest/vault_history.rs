use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, BlockTimestamp, CandleTimeframe},
    views::portfolio::{PortfolioCurvePoint, VaultHistoryStats},
};

/// Query parameters for vault history (indexer).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultHistory {
    /// Vault address.
    pub address: AccountAddress,
    /// Start timestamp in milliseconds (inclusive). Use 0 for all available history.
    pub from: BlockTimestamp,
    /// End timestamp in milliseconds (inclusive). Use 0 for the latest available snapshot.
    pub to: BlockTimestamp,
    /// Aggregation interval for the return and NAV curves.
    pub interval: CandleTimeframe,
}

/// Vault returns, NAV, TVL, annualized APR, and summary stats.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultHistoryResponse {
    /// Return curve derived from vault account equity snapshots.
    pub returns: Vec<PortfolioCurvePoint>,
    /// Per-share NAV curve normalized to 1.0 at the vault's first positive NAV.
    pub nav: Vec<PortfolioCurvePoint>,
    /// TVL curve derived from vault account equity snapshots.
    pub tvl: Vec<PortfolioCurvePoint>,
    /// Annualized percentage return as a decimal string.
    pub apr: String,
    /// Summary stats derived from the per-share NAV returns over the requested range.
    pub vault_stats: VaultHistoryStats,
}
