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

/// Messages sent from the server to connected clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// New quote for an instrument.
    QuoteUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Quote details.
        #[serde(rename = "quote")]
        quote: QuoteView,
    },
    /// Orderbook change.
    OrderbookUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// New state of the orderbook.
        orderbook: OrderbookView,
    },
    /// Update for the mark price of an instrument.
    MarkPriceUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// New mark price.
        #[serde(rename = "markPrice")]
        mark_price: MarkPriceView,
    },
    /// Update for the mark price of an asset.
    AssetMarkPriceUpdate {
        /// Asset whose mark price changed.
        #[serde(rename = "assetId")]
        asset_id: AssetId,
        /// New mark price.
        #[serde(rename = "markPrice")]
        mark_price: MarkPriceView,
    },
    /// Batch of order events optionally filtered by account or instrument.
    OrderEventUpdate {
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
        /// List of order events.
        #[serde(rename = "orderEvents")]
        order_events: Vec<OrderEventClientView>,
    },
    /// Collateral value update for an account.
    CollateralUpdate {
        /// Asset identifier for the collateral.
        #[serde(rename = "assetId")]
        asset_id: AssetId,
        /// Account whose collateral changed.
        account: AccountAddress,
        /// New collateral amount as a decimal string.
        collateral: String,
    },
    /// Positions update for an account.
    PositionUpdate {
        /// Account owning the positions.
        account: AccountAddress,
        /// Set of positions.
        positions: PositionSetView,
    },
    /// Risk metric update for an account.
    AccountRiskUpdate {
        /// Affected account.
        account: AccountAddress,
        /// Updated risk view.
        risk: AccountView,
    },
    /// Open orders update for an account.
    OpenOrdersUpdate {
        /// Account whose orders are reported.
        account: AccountAddress,
        /// List of orders.
        orders: Vec<OpenOrderView>,
    },
    /// Funding rate update for an instrument.
    FundingRateUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// New funding rate as a decimal string.
        #[serde(rename = "fundingRate")]
        funding_rate: String,
        /// Premium index value as a string.
        #[serde(rename = "premiumIndex")]
        premium_index: String,
        /// Timestamp of the update.
        timestamp: BlockTimestamp,
        /// Round number associated with the update.
        round: u64,
    },
    /// Instrument statistics update.
    InstrumentStatsUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Statistics details.
        stats: InstrumentStatsView,
    },
    /// New candle.
    CandleUpdate {
        /// Candle information.
        candle: CandleView,
    },
    /// Position funding payment update.
    PositionFundingUpdate {
        /// Account affected by the funding update.
        account: AccountAddress,
        /// Funding rate as a decimal string.
        #[serde(rename = "fundingRate")]
        funding_rate: String,
        /// Timestamp of the funding event.
        timestamp: BlockTimestamp,
        /// Round number for the funding update.
        round: u64,
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Profit/loss as a decimal string.
        pnl: String,
    },
    /// Last match price for an instrument.
    LastMatchPriceUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Last match price as a decimal string.
        #[serde(rename = "lastMatchPrice")]
        last_match_price: String,
    },
    /// Acknowledgement of subscription.
    SubscribeConfirmation(SubscriptionKind),
    /// Acknowledgement of unsubscription.
    UnsubscribeConfirmation(SubscriptionKind),
    /// Ping response.
    Pong,
    /// Error message from the server.
    Error(String),
}
