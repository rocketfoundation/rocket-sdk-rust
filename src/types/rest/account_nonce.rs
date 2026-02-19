use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Request params to fetch the current nonce for an account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountNonce {
    /// Address of the account whose nonce is requested.
    pub account: AccountAddress,
}

/// Response returning an account's transaction nonce.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountNonceResponse {
    /// The current transaction nonce for the account (0 for new accounts).
    pub nonce: u64,
}
