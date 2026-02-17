use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountNonce {
    /// Account address
    pub account: AccountAddress,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountNonceResponse {
    /// The current transaction nonce for the account (0 for new accounts)
    pub nonce: u64,
}
