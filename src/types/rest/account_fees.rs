use serde::{Deserialize, Serialize};

use crate::types::{primitives::AccountAddress, views::account_fees::AccountFeesClientView};

/// Request params to retrieve fee details for an account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountFees {
    /// Account address to query fees for.
    pub account: AccountAddress,
}

/// Response containing the fee information for an account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountFeesResponse {
    /// Fee details as returned by the server.
    #[serde(rename = "accountFees")]
    pub account_fees: AccountFeesClientView,
}
