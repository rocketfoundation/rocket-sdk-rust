use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::primitives::InstrumentId;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionView {
    pub quantity: String,
    pub average_price: String,
    pub liquidation_price: String,
    pub accrued_funding: String,
    pub unrealized_pnl: String,
    pub reserved_margin: String,
    pub leverage_setting: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PositionSetView(HashMap<InstrumentId, PositionView>);

impl AsRef<HashMap<InstrumentId, PositionView>> for PositionSetView {
    fn as_ref(&self) -> &HashMap<InstrumentId, PositionView> {
        &self.0
    }
}

impl AsMut<HashMap<InstrumentId, PositionView>> for PositionSetView {
    fn as_mut(&mut self) -> &mut HashMap<InstrumentId, PositionView> {
        &mut self.0
    }
}
