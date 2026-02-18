use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountView {
    pub account: AccountAddress,
    pub available_collateral: String,
    pub margin_requirement: String,
    pub unreserved_margin: String,
    pub equity: String,
    pub reserved_margin: String,
    pub unrealized_pnl: String,
    pub notional_value: String,
}
