use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, BlockTimestamp};

/// Request params to get faucet claim status for an account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetFaucetClaim {
    /// Account address.
    pub account: AccountAddress,
}

/// Response indicating the timestamp of the last faucet claim.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFaucetClaimResponse {
    /// Optional timestamp (ms) when the account last claimed from faucet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_claim_timestamp: Option<BlockTimestamp>,
}
