use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AccountAddress, BlockTimestamp, GlobalOrderId, InstrumentId, OrderSide,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrderView {
    pub order_id: GlobalOrderId,
    pub trader: AccountAddress,
    pub order_type: OrderType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    pub instrument_id: InstrumentId,
    pub price: String,
    pub quantity: String,
    pub filled_quantity: String,
    pub side: OrderSide,
    pub timestamp: BlockTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum TriggerType {
    StopLoss,
    TakeProfit,
}
