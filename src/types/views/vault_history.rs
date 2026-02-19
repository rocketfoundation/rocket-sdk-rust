use alloy_primitives::TxHash;
use serde::{Deserialize, Serialize};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, AssetId, BlockTimestamp, Round, Shares},
};

/// A vault history entry with human-readable values.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultHistoryEntryClientView {
    /// Type of event (Deposit or Withdraw).
    pub event_type: VaultHistoryEventType,
    /// Vault address.
    pub vault: AccountAddress,
    /// User who deposited/withdrew.
    pub user: AccountAddress,
    /// Asset deposited/withdrawn.
    pub asset_id: AssetId,
    /// Amount deposited or payout amount for withdrawals (human-readable).
    pub amount: String,
    /// Shares minted (for deposit) or burned (for withdraw).
    pub shares: Shares,
    /// Timestamp of the event.
    pub timestamp: BlockTimestamp,
    /// Round in which the event occurred.
    pub round: Round,
    /// Transaction hash.
    pub tx_hash: TxHash,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VaultHistoryEventType {
    Deposit,
    Withdraw,
}

/// View containing multiple vault history entries with human-readable values.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VaultHistoryClientView(Vec<VaultHistoryEntryClientView>);

impl_as_ref_mut_newtype!(VaultHistoryClientView, Vec<VaultHistoryEntryClientView>);
