use serde::{Deserialize, Serialize};

use crate::types::primitives::{AssetId, BlockTimestamp};

/// Market-maker protection limits for a (`account`, `mmp_tag`) pair.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarketMakerProtectionConfig {
    /// Underlying asset that protected orders must trade.
    pub mmp_underlying_asset_id: AssetId,
    /// Maximum filled quantity over `window`, summed over absolute fill sizes.
    /// `None` disables the quantity trigger.
    #[serde(alias = "notionalQuantityLimit")]
    pub quantity_limit: Option<u64>,
    /// Maximum absolute net transaction delta over `window`. `None` disables
    /// the delta trigger. Clearing every limit removes MMP for the account and tag.
    #[serde(default)]
    pub delta_limit: Option<u64>,
    /// Rolling timestamp window shared by the quantity and delta triggers.
    pub window: BlockTimestamp,
    /// Timestamp duration to freeze new/remaining orders after a limit triggers.
    pub freeze: BlockTimestamp,
    /// Maximum active protected quote quantity per instrument side in underlying units.
    pub mmp_max_quote_quantity: u64,
}
