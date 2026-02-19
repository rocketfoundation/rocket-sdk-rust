use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::InstrumentId};

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
    /// Reserved margin as a decimal string.
    pub reserved_margin: String,
    /// Leverge setting used for the position.
    pub leverage_setting: u64,
}

/// Set of positions indexed by instrument ID.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PositionSetView(HashMap<InstrumentId, PositionView>);

impl_as_ref_mut_newtype!(PositionSetView, HashMap<InstrumentId, PositionView>);
