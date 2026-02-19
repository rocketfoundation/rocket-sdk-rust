use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{InstrumentId, Round},
    rest::pagination::PaginationData,
    views::funding_rate::FundingRateView,
};

/// Request parameters for funding rate historical events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateEvents {
    /// Instrument identifier to query.
    pub instrument_id: InstrumentId,
    /// Optional starting round number.
    pub start_round: Option<Round>,
    /// Optional ending round number.
    pub end_round: Option<Round>,
    /// Pagination parameters.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Response containing a list of funding rate events.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateEventsResponse {
    /// Vector of funding rate views for each event.
    pub events: Vec<FundingRateView>,
    /// Pagination data.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
