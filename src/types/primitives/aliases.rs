/// A timestamp in milliseconds since Unix epoch.
pub type BlockTimestamp = u64;

/// Identifier for an asset.
pub type AssetId = usize;

/// Number of shares.
pub type Shares = u128;

/// An incremental round identifier.
pub type Round = u64;

/// Unique identifier assigned to each order.
pub type GlobalOrderId = u64;

/// Rank assigned to a liquidity provider, used to determine fee values.
pub type LiquidityProviderRank = u64;

/// Level of a liquidity provider tier; Signifies provider's number in the provider's list (by volume).
pub type LiquidityProviderTierLevel = u64;

/// Internal index of an order.
pub type OrderIx = usize;

/// Ticker symbol string for an asset.
pub type AssetTicker = String;

/// Unit of haircut expressed in ticks.
pub type HaircutTick = u32;

/// Scale factor for prices; negative values denote multiplication by the scale, positive denote division.
pub type PriceScale = i32;

/// Discrete tick size for price values.
pub type PriceTick = u64;

/// Scale factor for quantities; negative values denote multiplication by the scale, positive denote division.
pub type QuantityScale = i32;

/// Discrete tick for quantity values.
pub type QuantityTick = i64;

/// Fee rate represented in base units.
pub type FeeRate = u64;

/// Measured volume of traded assets.
pub type TradedVolume = u64;

/// Change in a scenario, can be positive or negative depending on context.
pub type ScenarioChange = i64;
