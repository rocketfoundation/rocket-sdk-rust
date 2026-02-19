use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{BlockTimestamp, InstrumentId},
};

/// Mapping of instrument identifiers to corresponding funding rate views.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct FundingRateByInstrumentClientView(HashMap<InstrumentId, FundingRateView>);

impl_as_ref_mut_newtype!(FundingRateByInstrumentClientView, HashMap<InstrumentId, FundingRateView>);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateView {
    /// Funding rate as a decimal string.
    pub funding_rate: String,
    /// Premium index as a decimal string.
    pub premium_index: String,
    /// Timestamp when the funding rate was recorded.
    pub timestamp: BlockTimestamp,
    /// Number of round in which the update happened.
    pub round: u64,
}
