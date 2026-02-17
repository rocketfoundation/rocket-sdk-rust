use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::bridge_event::BridgeEventsSetClientView,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBridgeEvents {
    pub account: Option<AccountAddress>,
    pub round_from: Option<String>,
    pub round_to: Option<String>,
    #[serde(flatten)]
    pub pagination_data: PaginationData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBridgeEventsResponse {
    pub events: BridgeEventsSetClientView,
}
