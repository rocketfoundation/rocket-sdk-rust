use serde::{Deserialize, Serialize};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, BlockTimestamp, InstrumentId},
};

/// Collection wrapper for position funding event views.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PositionFundingEventsClientViewSet(Vec<PositionFundingEventView>);

impl_as_ref_mut_newtype!(
    PositionFundingEventsClientViewSet,
    Vec<PositionFundingEventView>
);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionFundingEventView {
    /// Funding rate as a decimal string.
    pub funding_rate: String,
    /// Timestamp of the funding event.
    pub timestamp: BlockTimestamp,
    /// Round number associated with the event.
    pub round: u64,
    /// Instrument identifier involved.
    pub instrument_id: InstrumentId,
    /// Profit or loss as a decimal string.
    pub pnl: String,
    /// Account address affected.
    pub account: AccountAddress,
    /// Quantity of position involved as a string.
    pub position_quantity: String,
    /// Mark price of the instrument at event time.
    pub instrument_mark_price: String,
}
