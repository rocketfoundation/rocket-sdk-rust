use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{
        AccountAddress, AssetId, BlockTimestamp, FeeRate, GlobalOrderId, InstrumentId, OrderIx,
        PriceScale, PriceTick, QuantityScale, QuantityTick,
    },
    views::order_event::RejectionReason,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlaceOrderResult {
    Success(OrderEvent),
    Err(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub order_id: GlobalOrderId,
    pub order_ix: OrderIx,
    pub account: AccountAddress,
    pub instrument: InstrumentId,
    pub event_data: OrderEventData,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OrderEventData {
    Fill {
        price: PriceTick,
        size: QuantityTick,
        price_scale: PriceScale,
        quantity_scale: QuantityScale,
        settlement_asset: AssetId,
        pnl: Option<i64>,
        timestamp: BlockTimestamp,
        is_passive: bool,
        is_filled: bool,
        /// Remaining order size before this fill (legacy semantics).
        order_quantity: QuantityTick,
        /// Original order size captured at placement/last modify.
        original_order_quantity: QuantityTick,
        fee_rate: FeeRate,
        fee_amount: u64,
        abs_position_size_change: QuantityTick,
        is_liquidation: bool,
    },
    Placed {
        price: PriceTick,
        size: QuantityTick,
        price_scale: PriceScale,
        quantity_scale: QuantityScale,
        settlement_asset: AssetId,
        timestamp: BlockTimestamp,
        is_passive: bool,
        is_filled: bool,
        order_quantity: QuantityTick,
    },
    Canceled,
    Modified {
        price: PriceTick,
        size: QuantityTick,
        price_scale: PriceScale,
        quantity_scale: QuantityScale,
        timestamp: BlockTimestamp,
    },
    Rejected {
        reason: RejectionReason,
    },
}
