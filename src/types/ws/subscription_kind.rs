use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetId, CandleTimeframe, InstrumentId};

/// Market data subscription topics offered by the stream service.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionKind {
    Orderbook {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    PriceFeed {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    AssetPriceFeed {
        #[serde(rename = "assetId")]
        asset_id: AssetId,
    },
    OrderEvents {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<AccountAddress>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        instrument_id: Option<InstrumentId>,
    },
    Collateral {
        #[serde(rename = "assetId")]
        asset_id: AssetId,
        account: AccountAddress,
    },
    Position {
        account: AccountAddress,
    },
    AccountRisk {
        account: AccountAddress,
    },
    OpenOrders {
        account: AccountAddress,
    },
    FundingRate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    InstrumentStats {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
    Candle {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        interval: CandleTimeframe,
    },
    PositionFunding {
        account: AccountAddress,
    },
    LastMatchPrice {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
    },
}
