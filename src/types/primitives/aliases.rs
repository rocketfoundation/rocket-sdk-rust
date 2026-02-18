/// A timestamp in milliseconds since Unix epoch.
pub type BlockTimestamp = u64;
pub type AssetId = usize;
pub type Shares = u128;
/// A round identifier.
pub type Round = u64;
pub type GlobalOrderId = u64;
pub type LiquidityProviderRank = u64;
/// n -> n provider(s) are in that tier
pub type LiquidityProviderTierLevel = u64;
pub type OrderIx = usize;
pub type AssetTicker = String;
pub type HaircutTick = u32;

pub type PriceScale = i32; //-ve means *, +ve means /
pub type PriceTick = u64;

pub type QuantityScale = i32; //-ve means *, +ve means /
pub type QuantityTick = i64;

pub type FeeRate = u64;
pub type TradedVolume = u64;

pub type ScenarioChange = i64;
