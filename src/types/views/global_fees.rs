use serde::{Deserialize, Serialize};

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::LiquidityProviderTierLevel};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalFeesClientView {
    /// Fee ladder view.
    pub regular_fee_ladder: FeeLadderClientView,
    /// Liquidity provider fee ranking view.
    pub liquidity_provider_fee_ranking: LiquidityProviderFeeRankingClientView,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeeRatesClientView {
    /// Passive fee rate as a string.
    pub passive: String,
    /// Active fee rate as a string.
    pub active: String,
}

/// Client view of a fee ladder mapping thresholds to rates.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FeeLadderClientView(Vec<(String, FeeRatesClientView)>);

impl_as_ref_mut_newtype!(FeeLadderClientView, Vec<(String, FeeRatesClientView)>);

/// Liquidity provider fee rankings.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiquidityProviderFeeRankingClientView(
    Vec<(LiquidityProviderTierLevel, FeeRatesClientView)>,
);

impl_as_ref_mut_newtype!(
    LiquidityProviderFeeRankingClientView,
    Vec<(LiquidityProviderTierLevel, FeeRatesClientView)>
);
