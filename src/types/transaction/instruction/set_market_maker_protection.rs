use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, MMPTag, MarketMakerProtectionConfig};

/// Payload for configuring market-maker protection on an account and tag.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMarketMakerProtectionData {
    /// Account the MMP config applies to.
    pub to: AccountAddress,
    /// Protection tag the config is bound to.
    pub mmp_tag: MMPTag,
    /// Protection limits. Clearing every limit removes MMP for the account and tag.
    pub config: MarketMakerProtectionConfig,
}
