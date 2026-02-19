use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetId, CandleTimeframe, InstrumentId};

/// Market data subscription topics offered by the stream service.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionKind {
    /// Orderbook updates for a specific instrument.
    Orderbook {
        /// Instrument to subscribe to.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    /// Price feed updates for an instrument.
    PriceFeed {
        /// Instrument to subscribe to.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    /// Price feed updates for an asset.
    AssetPriceFeed {
        /// Asset to subscribe to.
        #[serde(rename = "assetId")]
        asset_id: AssetId,
    },
    /// Order events feed.
    OrderEvents {
        /// Optional account filter.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<AccountAddress>,
        /// Optional instrument filter.
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        instrument_id: Option<InstrumentId>,
    },
    /// Collateral quantity updates for an account and asset.
    Collateral {
        /// Asset identifier for collateral.
        #[serde(rename = "assetId")]
        asset_id: AssetId,
        /// Account whose collateral is monitored.
        account: AccountAddress,
    },
    /// Position updates for an account.
    Position {
        /// Account whose positions are tracked.
        account: AccountAddress,
    },
    /// Account risk updates.
    AccountRisk {
        /// Account to monitor for risk changes.
        account: AccountAddress,
    },
    /// Open orders for an account.
    OpenOrders {
        /// Account whose open orders are tracked.
        account: AccountAddress,
    },
    /// Funding rate updates for an instrument.
    FundingRate {
        /// Instrument identifier.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    /// Instrument statistics updates.
    InstrumentStats {
        /// Instrument identifier.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    /// Candle price data for an instrument.
    Candle {
        /// Instrument identifier.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Timeframe for the candles.
        interval: CandleTimeframe,
    },
    /// Position funding payment events.
    PositionFunding {
        /// Account affected by funding updates.
        account: AccountAddress,
    },
    /// Lst match price updates.
    LastMatchPrice {
        /// Instrument identifier.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
}
