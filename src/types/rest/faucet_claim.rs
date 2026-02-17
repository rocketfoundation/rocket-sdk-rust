use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, BlockTimestamp};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetFaucetClaim {
    pub account: AccountAddress,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFaucetClaimResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_claim_timestamp: Option<BlockTimestamp>,
}
