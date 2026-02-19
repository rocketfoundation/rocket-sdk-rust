use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, FeeRate, LiquidityProviderTierLevel, TradedVolume};

/// Payload for setting fee tiers.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeeTierData {
    /// Optional standard fee ladder configuration.
    pub regular_fee_ladder: Option<FeeLadder>,
    /// Optional liquidity provider fee ranking configuration.
    pub liquidity_provider_fee_ranking: Option<LiquidityProviderFeeRanking>,
}

/// Payload for updating the fee collector address.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFeeCollectorData {
    /// Account address designated to collect fees.
    pub fee_collector: AccountAddress,
}

/// Ordered mapping from traded volume thresholds to corresponding fee rates.
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct FeeLadder(
    /// Underlying map of volume thresholds to fee rates.
    pub BTreeMap<TradedVolume, FeeRates>,
);

/// Ranking information for liquidity provider fee rates.
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct LiquidityProviderFeeRanking(
    /// Vector of tier levels paired with fee rates.
    pub Vec<(LiquidityProviderTierLevel, FeeRates)>,
);

/// Fee rates for passive and active orders.
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct FeeRates {
    /// Fee rate applied to passive orders.
    pub passive: FeeRate,
    /// Fee rate applied to active orders.
    pub active: FeeRate,
}
