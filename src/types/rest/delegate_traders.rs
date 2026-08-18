use serde::{Deserialize, Serialize};

use crate::types::{primitives::AccountAddress, views::delegate_traders::DelegateTraderView};

/// Request params for delegate traders on the node.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDelegateTraders {
    /// Account whose delegates are requested.
    pub account: AccountAddress,
}

/// Delegate traders for the requested account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDelegateTradersResponse {
    pub delegates: Vec<DelegateTraderView>,
}
