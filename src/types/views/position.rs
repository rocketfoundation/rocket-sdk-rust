use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::{
        primitives::{BlockTimestamp, InstrumentId},
        views::instrument_type::AggregatedInstrumentType,
    },
};

/// Position information.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionView {
    /// Position quantity as a decimal string.
    pub quantity: String,
    /// Average entry price as a decimal string.
    pub average_price: String,
    /// Liquidation price as a decimal string.
    pub liquidation_price: String,
    /// Accrued funding amount as a decimal string.
    pub accrued_funding: String,
    /// Unrealized profit or loss as a decimal string.
    pub unrealized_pnl: String,
    /// Deprecated compatibility alias for `allocated_initial_margin`.
    pub reserved_margin: String,
    /// This position's additive share of the account's position margin.
    #[serde(default)]
    pub allocated_initial_margin: String,
    /// Margin required by this position before same-underlying offsets.
    #[serde(default)]
    pub standalone_initial_margin: String,
    /// Leverage setting used for the position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leverage_setting: Option<u64>,
    /// Timestamp when the position was opened.
    #[serde(default)]
    pub created_at: BlockTimestamp,
    /// Timestamp when the position was last updated.
    #[serde(default)]
    pub updated_at: BlockTimestamp,
    /// Aggregated instrument type.
    #[serde(default)]
    pub instrument_type: AggregatedInstrumentType,
    /// Instrument ticker.
    #[serde(default)]
    pub ticker: String,
    /// Underlying asset ticker.
    #[serde(default)]
    pub underlying_ticker: String,
}

/// Set of positions indexed by instrument ID.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PositionSetView(HashMap<InstrumentId, PositionView>);

impl_as_ref_mut_newtype!(PositionSetView, HashMap<InstrumentId, PositionView>);
