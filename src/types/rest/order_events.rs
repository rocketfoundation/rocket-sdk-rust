use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::order_event::OrderEventClientView,
};

/// Request params to fetch order events for a specific account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAccountOrderEvents {
    /// Account whose order events are requested.
    pub account: AccountAddress,
    /// Pagination parameters.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Response containing a list of order events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderEventsResponse {
    /// Vector of order event views, `None` if there are no events.
    #[serde(default)]
    pub order_events: Option<Vec<OrderEventClientView>>,
    /// Pagination data.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
