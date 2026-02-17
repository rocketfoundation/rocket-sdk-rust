use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    impl_as_ref_mut_newtype,
    types::primitives::{BlockTimestamp, InstrumentId},
};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct FundingRateByInstrumentClientView(HashMap<InstrumentId, FundingRateView>);

impl_as_ref_mut_newtype!(FundingRateByInstrumentClientView, HashMap<InstrumentId, FundingRateView>);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateView {
    pub funding_rate: String,
    pub premium_index: String,
    pub timestamp: BlockTimestamp,
    pub round: u64,
}
