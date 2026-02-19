use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::position_funding_events::PositionFundingEventsClientViewSet,
};

/// Request params to retrieve funding events for an account's positions.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAccountPositionFundingEvents {
    /// Account address to query.
    pub account: AccountAddress,
    /// Pagination data.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Response with a collection of position funding events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionFundingEventsResponse {
    /// Collection of funding events, `None` if there are no events.
    pub events: Option<PositionFundingEventsClientViewSet>,
    #[serde(flatten, default)]
    /// Pagination data.
    pub pagination_data: PaginationData,
}
