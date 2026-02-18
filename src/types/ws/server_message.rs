use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, AssetId, BlockTimestamp, InstrumentId},
    views::{
        account_risk::AccountView, candle::CandleView, instrument_stats::InstrumentStatsView,
        mark_price::MarkPriceView, open_order::OpenOrderView, order_event::OrderEventClientView,
        orderbook::OrderbookView, position::PositionSetView, quote::QuoteView,
    },
    ws::subscription_kind::SubscriptionKind,
};

/// Enumerates messages sent from the server to connected clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    QuoteUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        #[serde(rename = "quote")]
        quote: QuoteView,
    },
    OrderbookUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        orderbook: OrderbookView,
    },
    MarkPriceUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        #[serde(rename = "markPrice")]
        mark_price: MarkPriceView,
    },
    AssetMarkPriceUpdate {
        #[serde(rename = "assetId")]
        asset_id: AssetId,
        #[serde(rename = "markPrice")]
        mark_price: MarkPriceView,
    },
    OrderEventUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<AccountAddress>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        instrument_id: Option<InstrumentId>,
        #[serde(rename = "orderEvents")]
        order_events: Vec<OrderEventClientView>,
    },
    CollateralUpdate {
        #[serde(rename = "assetId")]
        asset_id: AssetId,
        account: AccountAddress,
        collateral: String,
    },
    PositionUpdate {
        account: AccountAddress,
        positions: PositionSetView,
    },
    AccountRiskUpdate {
        account: AccountAddress,
        risk: AccountView,
    },
    OpenOrdersUpdate {
        account: AccountAddress,
        orders: Vec<OpenOrderView>,
    },
    FundingRateUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        #[serde(rename = "fundingRate")]
        funding_rate: String,
        #[serde(rename = "premiumIndex")]
        premium_index: String,
        timestamp: BlockTimestamp,
        round: u64,
    },
    InstrumentStatsUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        stats: InstrumentStatsView,
    },
    CandleUpdate {
        candle: CandleView,
    },
    PositionFundingUpdate {
        account: AccountAddress,
        #[serde(rename = "fundingRate")]
        funding_rate: String,
        timestamp: BlockTimestamp,
        round: u64,
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        pnl: String,
    },
    LastMatchPriceUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        #[serde(rename = "lastMatchPrice")]
        last_match_price: String,
    },
    SubscribeConfirmation(SubscriptionKind),
    UnsubscribeConfirmation(SubscriptionKind),
    Pong,
    Error(String),
}
