use serde::{Deserialize, Serialize};

use crate::types::primitives::{
    AccountAddress, AssetId, BlockTimestamp, GlobalOrderId, InstrumentId,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderEventClientView {
    pub order_id: GlobalOrderId,
    pub account: AccountAddress,
    pub instrument: InstrumentId,
    pub event_data: OrderEventDataClientView,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderEventDataClientView {
    Fill {
        price: String,
        size: String,
        remaining_size: String,
        original_size: String,
        settlement_asset: AssetId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pnl: Option<String>,
        timestamp: BlockTimestamp,
        is_passive: bool,
        is_filled: bool,
        fee_rate: String,
        fee_amount: String,
        is_liquidation: bool,
    },
    Placed {
        price: String,
        size: String,
        remaining_size: String,
        original_size: String,
        settlement_asset: AssetId,
        timestamp: BlockTimestamp,
        is_passive: bool,
        is_filled: bool,
    },
    Canceled,
    Modified {
        price: String,
        size: String,
        timestamp: BlockTimestamp,
    },
    Rejected {
        reason: RejectionReason,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RejectionReason {
    MarginViolated,
    NotEnoughLiquidity,
    TooMuchSlippage,
}
