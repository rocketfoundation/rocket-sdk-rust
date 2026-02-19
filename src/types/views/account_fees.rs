use serde::{Deserialize, Serialize};

use crate::types::primitives::LiquidityProviderRank;

/// Account fee information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFeesClientView {
    /// Passive fee rate as a decimal string.
    pub passive: String,
    /// Active fee rate as a decimal string.
    pub active: String,
    /// Total trading volume as a decimal string.
    pub total_volume: String,
    /// Rank of the liquidity provider.
    pub liquidity_provider_rank: LiquidityProviderRank,
    /// Fee tier.
    pub tier: String,
}
