use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::order_event::OrderEventClientView,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetAccountOrderEvents {
    pub account: AccountAddress,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderEventsResponse {
    pub order_events: Option<Vec<OrderEventClientView>>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
