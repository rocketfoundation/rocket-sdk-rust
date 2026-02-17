use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::vault_history::VaultHistoryClientView,
};

/// Query parameters for vault history
/// Vault is required, user is optional
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultHistory {
    /// Vault address
    pub vault: AccountAddress,
    /// User address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AccountAddress>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Response containing vault history entries with human-readable values
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultHistoryResponse {
    pub history: VaultHistoryClientView,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
