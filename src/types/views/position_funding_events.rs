use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, BlockTimestamp, InstrumentId};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PositionFundingEventsClientViewSet(Vec<PositionFundingEventView>);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionFundingEventView {
    pub funding_rate: String,
    pub timestamp: BlockTimestamp,
    pub round: u64,
    pub instrument_id: InstrumentId,
    pub pnl: String,
    pub account: AccountAddress,
    pub position_quantity: String,
    pub instrument_mark_price: String,
}
