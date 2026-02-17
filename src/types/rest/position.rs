use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, InstrumentId},
    views::position::PositionSetView,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetPosition {
    pub account: AccountAddress,
    #[serde(rename = "instrumentId", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<InstrumentId>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPositionsResponse {
    pub positions: Option<PositionSetView>,
}
