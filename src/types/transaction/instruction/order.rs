use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, GlobalOrderId, InstrumentId},
};

/// Wrapper for a vector of order requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct OrderRequestSet(
    /// Underlying list of order requests.
    pub Vec<OrderRequest>,
);

impl Deref for OrderRequestSet {
    type Target = Vec<OrderRequest>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OrderRequestSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<OrderRequest>> for OrderRequestSet {
    fn from(vec: Vec<OrderRequest>) -> Self {
        OrderRequestSet(vec)
    }
}

impl From<OrderRequestSet> for Vec<OrderRequest> {
    fn from(set: OrderRequestSet) -> Self {
        set.0
    }
}

impl_as_ref_mut_newtype!(OrderRequestSet, Vec<OrderRequest>);

/// Variants representing different kinds of order actions.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderRequest {
    /// Place a limit order.
    Limit(PlaceLimitOrderRequest),
    /// Place a market order.
    Market(PlaceMarketOrderRequest),
    /// Cancel a specific order.
    Cancel(CancelOrderRequest),
    /// Cancel all orders (optionally filtered by instrument).
    CancelAll(CancelAllOrderRequest),
    /// Modify an existing order.
    Modify(ModifyOrderRequest),
}

/// Side of an order, either buy or sell.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceLimitOrderRequest {
    /// Identifier of the instrument.
    pub instrument_id: InstrumentId,
    /// Side of the order.
    pub side: OrderSide,
    /// Price at which to place the order (decimal string).
    pub price: String,
    /// Quantity to trade (decimal string).
    pub quantity: String,
    /// Address of the trader placing the order.
    pub trader: AccountAddress,
    /// Optional trigger price for conditional orders.
    pub trigger_price: Option<String>,
    /// If true, the order is reduce-only.
    pub reduce_only: bool,
    /// If true, the order is a take-profit order.
    pub take_profit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceMarketOrderRequest {
    /// Identifier of the instrument.
    pub instrument_id: InstrumentId,
    /// Side of the order.
    pub side: OrderSide,
    /// Quantity to trade (decimal string).
    pub quantity: String,
    /// Address of the trader placing the order.
    pub trader: AccountAddress,
    /// Optional trigger price for conditional orders.
    pub trigger_price: Option<String>,
    /// If true, the order is reduce-only.
    pub reduce_only: bool,
    /// If true, the order is a take-profit order.
    pub take_profit: bool,
    /// Optional maximum slippage allowed (decimal string), default "0.03" (0.03%).
    pub max_slippage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Identifier of the order to cancel.
    #[serde(rename = "orderId")]
    pub order_id: GlobalOrderId,
    /// Address of the trader who owns the order.
    pub trader: AccountAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderRequest {
    /// Optional instrument filter for cancellation.
    pub instrument_id: Option<InstrumentId>,
    /// Address of the trader canceling orders.
    pub trader: AccountAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderRequest {
    /// Identifier of the order to modify.
    pub order_id: GlobalOrderId,
    /// New price for the order (decimal string).
    pub new_price: String,
    /// Address of the trader modifying the order.
    pub trader: AccountAddress,
    /// New quantity for the order (decimal string).
    pub new_quantity: String,
    /// New optional trigger price.
    pub new_trigger_price: Option<String>,
}
