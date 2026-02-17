use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{InstrumentId, Round},
    rest::pagination::PaginationData,
    views::funding_rate::FundingRateView,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateEvents {
    pub instrument_id: InstrumentId,
    pub start_round: Option<Round>,
    pub end_round: Option<Round>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateEventsResponse {
    pub events: Vec<FundingRateView>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
