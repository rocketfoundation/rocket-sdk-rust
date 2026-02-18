use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::InstrumentId};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct InstrumentsSetView(HashMap<InstrumentId, InstrumentView>);

impl_as_ref_mut_newtype!(InstrumentsSetView, HashMap<InstrumentId, InstrumentView>);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentView {
    pub id: String,
    pub ticker: String,
    pub instrument_type: String,
    pub underlying_asset: String,
    pub settlement_asset: String,
    pub is_trading: bool,
    pub expiry: Option<String>,
    pub strike: Option<String>,
    pub price_scale: i32,
    pub quantity_scale: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worst_case_price_move_pct: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
    pub last_match_price: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worst_case_price_move_pct_margin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_leverage_margin: Option<String>,
}
