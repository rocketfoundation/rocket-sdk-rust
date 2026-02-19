use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::InstrumentId};

/// Set of instruments mapped by their identifiers.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct InstrumentsSetView(HashMap<InstrumentId, InstrumentView>);

impl_as_ref_mut_newtype!(InstrumentsSetView, HashMap<InstrumentId, InstrumentView>);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentView {
    /// Identifier of the instrument.
    pub id: String,
    /// Ticker symbol of the instrument.
    pub ticker: String,
    /// Type of the instrument (spot, perpetual, etc.).
    pub instrument_type: String,
    /// Underlying asset ticker.
    pub underlying_asset: String,
    /// Settlement asset ticker.
    pub settlement_asset: String,
    /// Whether trading is currently enabled.
    pub is_trading: bool,
    /// Expiry timestamp as a string (if there is one).
    pub expiry: Option<String>,
    /// Strike price as a string (if there is one).
    pub strike: Option<String>,
    /// Price scale factor.
    pub price_scale: i32,
    /// Quantity scale factor.
    pub quantity_scale: i32,
    /// Worst-case price move percentage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worst_case_price_move_pct: Option<String>,
    /// Maximum leverage as a string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
    /// Last matched price as a string.
    pub last_match_price: String,
    /// Worst-case price move percentage margin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worst_case_price_move_pct_margin: Option<String>,
    /// Max leverage margin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_leverage_margin: Option<String>,
}
