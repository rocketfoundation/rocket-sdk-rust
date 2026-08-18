use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Query parameters for an account's current vault positions.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultPortfolio {
    /// Account address whose vault positions should be returned.
    pub account: AccountAddress,
    /// Optional vault address filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault: Option<AccountAddress>,
}

/// Current vault positions for the requested account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultPortfolioResponse {
    pub positions: Vec<VaultPortfolioPosition>,
}

/// A single vault position for an account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VaultPortfolioPosition {
    pub vault: AccountAddress,
    /// Current account share balance in this vault.
    pub shares: String,
    /// Current total outstanding shares for this vault.
    pub total_shares: String,
    /// Latest indexed vault equity.
    pub vault_equity: String,
    /// Current value of the account's shares.
    pub value: String,
    /// Total deposits recorded for this account/vault.
    pub deposits: String,
    /// Total withdrawals recorded for this account/vault.
    pub withdrawals: String,
    /// Current vault PnL: `value + withdrawals - deposits`.
    pub pnl: String,
}
