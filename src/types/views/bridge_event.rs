use serde::{Deserialize, Serialize};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, BlockTimestamp},
};

/// Bridge event details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeEventClientView {
    /// Round number associated with the bridge event.
    pub round: u64,
    /// Timestamp of the event.
    pub timestamp: BlockTimestamp,
    /// Account address involved in the event.
    pub account_address: AccountAddress,
    /// External blockchain address.
    pub external_address: String,
    /// External token contract address.
    pub external_token_address: String,
    /// Identifier for the deposit or nonce for withdrawal.
    pub id: u64,
    /// Amount moved as a decimal string.
    pub amount: String,
    /// Type of the bridge event.
    pub event_type: BridgeEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BridgeEventType {
    /// Tokens were deposited into rocket chain.
    Deposit,
    /// Tokens were withdrawn from rocket chain.
    Withdraw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wrapper containing a list of bridge event views.
pub struct BridgeEventsSetClientView(Vec<BridgeEventClientView>);

impl_as_ref_mut_newtype!(BridgeEventsSetClientView, Vec<BridgeEventClientView>);
