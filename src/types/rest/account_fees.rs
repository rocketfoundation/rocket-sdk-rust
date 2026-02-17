use serde::{Deserialize, Serialize};

use crate::types::{primitives::AccountAddress, views::account_fees::AccountFeesClientView};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountFees {
    /// Account address
    pub account: AccountAddress,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountFeesResponse {
    #[serde(rename = "accountFees")]
    pub account_fees: AccountFeesClientView,
}
