use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, AssetId, BlockTimestamp, Round},
    rest::pagination::PaginationData,
};

/// Query parameters for indexer vault flow events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultEvents {
    /// Optional vault address filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault: Option<AccountAddress>,
    /// Optional account address filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountAddress>,
    /// Optional start round (inclusive). If provided, `roundTo` should also be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round_from: Option<String>,
    /// Optional end round (inclusive). If provided, `roundFrom` should also be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round_to: Option<String>,
    /// Pagination params.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Vault events returned for the selected filters.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVaultEventsResponse {
    pub events: Vec<VaultEventResponseItem>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// A single vault deposit/withdraw event.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VaultEventResponseItem {
    pub timestamp: BlockTimestamp,
    pub round: Round,
    pub tx_index: u32,
    pub event_index: u16,
    pub vault: AccountAddress,
    pub account: AccountAddress,
    /// Event type exposed to clients (`deposit` or `withdraw`).
    pub event_type: String,
    pub asset_id: AssetId,
    /// Absolute collateral amount in dollar units.
    pub amount: String,
    /// Signed share delta stored in the vault event row.
    pub shares: String,
    /// Transaction hash as a hex string.
    pub tx_hash: String,
}
