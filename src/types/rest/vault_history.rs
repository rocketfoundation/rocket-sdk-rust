use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::vault_history::VaultHistoryClientView,
};

/// Query parameters for vault history.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultHistory {
    /// Vault address.
    pub vault: AccountAddress,
    /// User address (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AccountAddress>,
    /// Pagination params.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Response containing vault history entries with human-readable values.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultHistoryResponse {
    /// Collection of vault events.
    pub history: VaultHistoryClientView,
    /// Pagination data from the request.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
