use serde::{Deserialize, Serialize};

use crate::{
    impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, BlockTimestamp},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeEventClientView {
    pub round: u64,
    pub timestamp: BlockTimestamp,
    pub account_address: AccountAddress,
    pub external_address: String,
    pub external_token_address: String,
    pub id: u64, // Deposit ID for deposit, nonce for withdraw
    pub amount: String,
    pub event_type: BridgeEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BridgeEventType {
    Deposit,
    Withdraw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeEventsSetClientView(Vec<BridgeEventClientView>);

impl_as_ref_mut_newtype!(BridgeEventsSetClientView, Vec<BridgeEventClientView>);
