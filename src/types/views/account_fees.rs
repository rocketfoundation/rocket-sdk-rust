use serde::{Deserialize, Serialize};

use crate::types::primitives::LiquidityProviderRank;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFeesClientView {
    pub passive: String,
    pub active: String,
    pub total_volume: String,
    pub liquidity_provider_rank: LiquidityProviderRank,
    pub tier: String,
}
