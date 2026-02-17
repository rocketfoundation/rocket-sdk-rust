use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData,
    views::position_funding_events::PositionFundingEventsClientViewSet,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetAccountPositionFundingEvents {
    pub account: AccountAddress,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionFundingEventsResponse {
    pub events: Option<PositionFundingEventsClientViewSet>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
