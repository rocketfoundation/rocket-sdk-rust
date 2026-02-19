use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::bridge_event::BridgeEventsSetClientView,
};

/// Request parameters for querying bridge events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBridgeEvents {
    /// Optional account filter for events.
    pub account: Option<AccountAddress>,
    /// Optional starting round for events.
    pub round_from: Option<String>,
    /// Optional ending round for events.
    pub round_to: Option<String>,
    /// Pagination settings for the query.
    #[serde(flatten)]
    pub pagination_data: PaginationData,
}

/// Response containing bridge events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBridgeEventsResponse {
    /// Set of bridge event views.
    pub events: BridgeEventsSetClientView,
}
