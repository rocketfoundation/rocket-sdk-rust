use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, InstrumentId},
    views::position::PositionSetView,
};

/// Request params to fetch positions for an account, optionally filtered by instrument.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetPosition {
    /// Account whose positions are requested.
    pub account: AccountAddress,
    /// Optional instrument filter.
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instrument_id: Option<InstrumentId>,
}

/// Response containing a collection of positions.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetPositionsResponse {
    /// Collection of positions, `None` if there are no positions.
    pub positions: Option<PositionSetView>,
}
