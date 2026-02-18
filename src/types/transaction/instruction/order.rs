use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
    impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, GlobalOrderId, InstrumentId},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct OrderRequestSet(pub Vec<OrderRequest>);

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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderRequest {
    Limit(PlaceLimitOrderRequest),
    Market(PlaceMarketOrderRequest),
    Cancel(CancelOrderRequest),
    CancelAll(CancelAllOrderRequest),
    Modify(ModifyOrderRequest),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceLimitOrderRequest {
    pub instrument_id: InstrumentId,
    pub side: OrderSide,
    /// String containing a decimal number
    pub price: String,
    /// String containing a decimal number
    pub quantity: String,
    pub trader: AccountAddress,
    pub trigger_price: Option<String>,
    pub reduce_only: bool,
    pub take_profit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceMarketOrderRequest {
    pub instrument_id: InstrumentId,
    pub side: OrderSide,
    /// String containing a decimal number
    pub quantity: String,
    pub trader: AccountAddress,
    pub trigger_price: Option<String>,
    pub reduce_only: bool,
    pub take_profit: bool,
    /// String containing a non-negative decimal number
    pub max_slippage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    #[serde(rename = "orderId")]
    pub order_id: GlobalOrderId,
    pub trader: AccountAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderRequest {
    pub instrument_id: Option<InstrumentId>,
    pub trader: AccountAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderRequest {
    pub order_id: GlobalOrderId,
    /// String containing a decimal number
    pub new_price: String,
    pub trader: AccountAddress,
    /// String containing a decimal number
    pub new_quantity: String,
    pub new_trigger_price: Option<String>,
}
