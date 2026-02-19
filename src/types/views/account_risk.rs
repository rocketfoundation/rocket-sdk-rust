use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Risk metrics for an account.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountView {
    /// Account address.
    pub account: AccountAddress,
    /// Amount of collateral available as a decimal string.
    pub available_collateral: String,
    /// Margin requirement as a decimal string.
    pub margin_requirement: String,
    /// Unreserved margin amount as a decimal string.
    pub unreserved_margin: String,
    /// Total equity as a decimal string.
    pub equity: String,
    /// Reserved margin as a decimal string.
    pub reserved_margin: String,
    /// Unrealized profit or loss as a decimal string.
    pub unrealized_pnl: String,
    /// Notional value of positions as a decimal string.
    pub notional_value: String,
}
