use serde::{Deserialize, Serialize};

use crate::{impl_as_ref_mut_newtype, types::primitives::LiquidityProviderTierLevel};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalFeesClientView {
    pub regular_fee_ladder: FeeLadderClientView,
    pub liquidity_provider_fee_ranking: LiquidityProviderFeeRankingClientView,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeeRatesClientView {
    pub passive: String,
    pub active: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FeeLadderClientView(Vec<(String, FeeRatesClientView)>);

impl_as_ref_mut_newtype!(FeeLadderClientView, Vec<(String, FeeRatesClientView)>);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiquidityProviderFeeRankingClientView(
    Vec<(LiquidityProviderTierLevel, FeeRatesClientView)>,
);

impl_as_ref_mut_newtype!(
    LiquidityProviderFeeRankingClientView,
    Vec<(LiquidityProviderTierLevel, FeeRatesClientView)>
);
