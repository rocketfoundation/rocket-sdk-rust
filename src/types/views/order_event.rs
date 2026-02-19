use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AccountAddress, AssetId, BlockTimestamp, GlobalOrderId, InstrumentId,
};

/// Order event info.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEventClientView {
    /// Identifier for the order.
    pub order_id: GlobalOrderId,
    /// Account associated with the event.
    pub account: AccountAddress,
    /// Instrument involved in the event.
    pub instrument: InstrumentId,
    /// Detailed event data.
    pub event_data: OrderEventDataClientView,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderEventDataClientView {
    /// Order was filled (fully or partially).
    Fill {
        /// Fill price as a string.
        price: String,
        /// Amount filled as a string.
        size: String,
        /// Remaining order size after fill.
        remaining_size: String,
        /// Original order size.
        original_size: String,
        /// Asset used for settlement.
        settlement_asset: AssetId,
        /// Profit/loss as string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pnl: Option<String>,
        /// Timestamp of the fill.
        timestamp: BlockTimestamp,
        /// Whether the order was passive.
        is_passive: bool,
        /// Whether the order was fully filled.
        is_filled: bool,
        /// Fee rate as a string.
        fee_rate: String,
        /// Fee amount as a string.
        fee_amount: String,
        /// Whether this was a liquidation fill.
        is_liquidation: bool,
    },
    /// Order was placed but not yet executed.
    Placed {
        /// Price set on the order.
        price: String,
        /// Size of the order.
        size: String,
        /// Remaining size of the order.
        remaining_size: String,
        /// Original size of the order.
        original_size: String,
        /// Asset used for settlement.
        settlement_asset: AssetId,
        /// Timestamp of placement.
        timestamp: BlockTimestamp,
        /// Whether the order is passive.
        is_passive: bool,
        /// Whether the order is immediately filled.
        is_filled: bool,
    },
    /// Order was cancelled.
    Canceled,
    /// Order was modified with new parameters.
    Modified {
        /// New price value.
        price: String,
        /// New size value.
        size: String,
        /// Timestamp of modification.
        timestamp: BlockTimestamp,
    },
    /// Order was rejected.
    Rejected {
        /// Reason for rejection.
        reason: RejectionReason,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RejectionReason {
    /// Rejected due to margin requirement violation.
    MarginViolated,
    /// Rejected because there was insufficient liquidity.
    NotEnoughLiquidity,
    /// Rejected due to excessive slippage.
    TooMuchSlippage,
}
