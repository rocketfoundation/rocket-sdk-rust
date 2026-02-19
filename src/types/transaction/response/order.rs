use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{
        AccountAddress, AssetId, BlockTimestamp, FeeRate, GlobalOrderId, InstrumentId, OrderIx,
        PriceScale, PriceTick, QuantityScale, QuantityTick,
    },
    views::order_event::RejectionReason,
};

/// Result of attempting to place an order.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlaceOrderResult {
    /// Order placing was successful and produced an event.
    Success(OrderEvent),
    /// Order placing failed with an error message.
    Err(String),
}

/// Event data returned when an order action occurs.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    /// Order identifier.
    pub order_id: GlobalOrderId,
    /// Internal order index.
    pub order_ix: OrderIx,
    /// Account that placed or modified the order.
    pub account: AccountAddress,
    /// Instrument associated with the order.
    pub instrument: InstrumentId,
    /// Specific details about what happened to the order.
    pub event_data: OrderEventData,
}

/// Detailed data for an order event.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OrderEventData {
    /// The order was filled (possibly partially).
    Fill {
        /// Price at which fill occurred.
        price: PriceTick,
        /// Quantity filled.
        size: QuantityTick,
        /// Scale factor for the price.
        price_scale: PriceScale,
        /// Scale factor for the quantity.
        quantity_scale: QuantityScale,
        /// Asset used for settlement.
        settlement_asset: AssetId,
        /// Profit/loss.
        pnl: Option<i64>,
        /// Timestamp when the fill happened.
        timestamp: BlockTimestamp,
        /// Whether the order was passive.
        is_passive: bool,
        /// Whether the order was completely filled.
        is_filled: bool,
        /// Remaining order size before this fill (legacy semantics).
        order_quantity: QuantityTick,
        /// Original order size captured at placement/last modify.
        original_order_quantity: QuantityTick,
        /// Fee rate applied to the fill.
        fee_rate: FeeRate,
        /// Fee amount charged.
        fee_amount: u64,
        /// Absolute change in position size.
        abs_position_size_change: QuantityTick,
        /// Indicates if the fill was part of a liquidation.
        is_liquidation: bool,
    },
    /// The order has been placed but not yet matched.
    Placed {
        /// Order price.
        price: PriceTick,
        /// Order quantity.
        size: QuantityTick,
        /// Scale factor for the price.
        price_scale: PriceScale,
        /// Scale factor for the quantity.
        quantity_scale: QuantityScale,
        /// Asset used for settlement.
        settlement_asset: AssetId,
        /// Timestamp of placement.
        timestamp: BlockTimestamp,
        /// Whether the order is passive.
        is_passive: bool,
        /// Whether the order is filled immediately.
        is_filled: bool,
        /// Remaining order size at placement.
        order_quantity: QuantityTick,
    },
    /// The order was canceled.
    Canceled,
    /// The order was modified with new parameters.
    Modified {
        /// New price for the order.
        price: PriceTick,
        /// New quantity for the order.
        size: QuantityTick,
        /// Scale factor for the new price.
        price_scale: PriceScale,
        /// Scale factor for the new quantity.
        quantity_scale: QuantityScale,
        /// Timestamp of modification.
        timestamp: BlockTimestamp,
    },
    /// The order was rejected with a reason.
    Rejected {
        /// Reason describing why the order was rejected.
        reason: RejectionReason,
    },
}
