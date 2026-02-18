use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, FeeRate, LiquidityProviderTierLevel, TradedVolume};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeeTierData {
    pub regular_fee_ladder: Option<FeeLadder>,
    pub liquidity_provider_fee_ranking: Option<LiquidityProviderFeeRanking>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeeCollectorData {
    pub fee_collector: AccountAddress,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct FeeLadder(pub BTreeMap<TradedVolume, FeeRates>);

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct LiquidityProviderFeeRanking(pub Vec<(LiquidityProviderTierLevel, FeeRates)>);

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct FeeRates {
    pub passive: FeeRate,
    pub active: FeeRate,
}
