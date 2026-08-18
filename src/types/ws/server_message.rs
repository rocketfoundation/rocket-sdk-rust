use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::types::{
    primitives::{AccountAddress, AssetId, BlockTimestamp, InstrumentId},
    transaction::{response::TransactionResponse, signature::Signature},
    views::{
        account_risk::AccountView,
        auction_fill::AuctionFillEntry,
        candle::CandleView,
        instrument_stats::InstrumentStatsView,
        mark_price::MarkPriceView,
        open_order::OpenOrderView,
        order_event::OrderEventClientView,
        orderbook::OrderbookView,
        position::PositionSetView,
        quote::{QuoteView, TickerView},
    },
    ws::subscription_kind::SubscriptionKind,
};

/// Messages sent from the server to connected clients.
#[derive(Debug, Clone, Serialize)]
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
    /// Auction fill batch for an instrument.
    AuctionFillUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Shared auction price as a decimal string.
        price: String,
        /// Compact fills in the batch.
        fills: Vec<AuctionFillEntry>,
    },
    /// Compact ticker update.
    TickerUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Compact ticker payload.
        #[serde(rename = "ticker")]
        ticker: TickerView,
    },
    /// Instrument statistics update.
    InstrumentStatsUpdate {
        /// Instrument id.
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        /// Statistics details.
        #[serde(rename = "instrumentStats")]
        instrument_stats: InstrumentStatsView,
    },
    /// Acknowledgement of subscription.
    SubscribeConfirmation(SubscriptionKind),
    /// Acknowledgement of unsubscription.
    UnsubscribeConfirmation(SubscriptionKind),
    /// Immediate result of a WebSocket `SubmitTransaction`.
    TransactionResult {
        signature: Signature,
        response: TransactionResponse,
    },
    /// Confirmation that a Deferred was stored for execute-on-disconnect.
    ExecuteOnDisconnectRegistered {
        signature: Signature,
        #[serde(rename = "deferredNonce")]
        deferred_nonce: u64,
    },
    /// Confirmation that any execute-on-disconnect registration for this connection was cleared.
    ExecuteOnDisconnectCleared,
    /// Ping response.
    Pong,
    /// Error message from the server.
    Error(String),
}

/// Tagged wire form used so deserialization does not recurse through [`ServerMessage`].
#[derive(Deserialize)]
enum ServerMessageTagged {
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
        #[serde(default, rename = "account")]
        account: Option<AccountAddress>,
        #[serde(default, rename = "instrumentId")]
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
    AuctionFillUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        price: String,
        fills: Vec<AuctionFillEntry>,
    },
    TickerUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        #[serde(rename = "ticker")]
        ticker: TickerView,
    },
    InstrumentStatsUpdate {
        #[serde(rename = "instrumentId")]
        instrument_id: InstrumentId,
        #[serde(rename = "instrumentStats", alias = "stats")]
        instrument_stats: InstrumentStatsView,
    },
    SubscribeConfirmation(SubscriptionKind),
    UnsubscribeConfirmation(SubscriptionKind),
    TransactionResult {
        signature: Signature,
        response: TransactionResponse,
    },
    ExecuteOnDisconnectRegistered {
        signature: Signature,
        #[serde(rename = "deferredNonce")]
        deferred_nonce: u64,
    },
    ExecuteOnDisconnectCleared,
    Pong,
    Error(String),
}

impl From<ServerMessageTagged> for ServerMessage {
    fn from(value: ServerMessageTagged) -> Self {
        match value {
            ServerMessageTagged::QuoteUpdate {
                instrument_id,
                quote,
            } => ServerMessage::QuoteUpdate {
                instrument_id,
                quote,
            },
            ServerMessageTagged::OrderbookUpdate {
                instrument_id,
                orderbook,
            } => ServerMessage::OrderbookUpdate {
                instrument_id,
                orderbook,
            },
            ServerMessageTagged::MarkPriceUpdate {
                instrument_id,
                mark_price,
            } => ServerMessage::MarkPriceUpdate {
                instrument_id,
                mark_price,
            },
            ServerMessageTagged::AssetMarkPriceUpdate {
                asset_id,
                mark_price,
            } => ServerMessage::AssetMarkPriceUpdate {
                asset_id,
                mark_price,
            },
            ServerMessageTagged::OrderEventUpdate {
                account,
                instrument_id,
                order_events,
            } => ServerMessage::OrderEventUpdate {
                account,
                instrument_id,
                order_events,
            },
            ServerMessageTagged::CollateralUpdate {
                asset_id,
                account,
                collateral,
            } => ServerMessage::CollateralUpdate {
                asset_id,
                account,
                collateral,
            },
            ServerMessageTagged::PositionUpdate { account, positions } => {
                ServerMessage::PositionUpdate { account, positions }
            }
            ServerMessageTagged::AccountRiskUpdate { account, risk } => {
                ServerMessage::AccountRiskUpdate { account, risk }
            }
            ServerMessageTagged::OpenOrdersUpdate { account, orders } => {
                ServerMessage::OpenOrdersUpdate { account, orders }
            }
            ServerMessageTagged::FundingRateUpdate {
                instrument_id,
                funding_rate,
                premium_index,
                timestamp,
                round,
            } => ServerMessage::FundingRateUpdate {
                instrument_id,
                funding_rate,
                premium_index,
                timestamp,
                round,
            },
            ServerMessageTagged::CandleUpdate { candle } => ServerMessage::CandleUpdate { candle },
            ServerMessageTagged::PositionFundingUpdate {
                account,
                funding_rate,
                timestamp,
                round,
                instrument_id,
                pnl,
            } => ServerMessage::PositionFundingUpdate {
                account,
                funding_rate,
                timestamp,
                round,
                instrument_id,
                pnl,
            },
            ServerMessageTagged::LastMatchPriceUpdate {
                instrument_id,
                last_match_price,
            } => ServerMessage::LastMatchPriceUpdate {
                instrument_id,
                last_match_price,
            },
            ServerMessageTagged::AuctionFillUpdate {
                instrument_id,
                price,
                fills,
            } => ServerMessage::AuctionFillUpdate {
                instrument_id,
                price,
                fills,
            },
            ServerMessageTagged::TickerUpdate {
                instrument_id,
                ticker,
            } => ServerMessage::TickerUpdate {
                instrument_id,
                ticker,
            },
            ServerMessageTagged::InstrumentStatsUpdate {
                instrument_id,
                instrument_stats,
            } => ServerMessage::InstrumentStatsUpdate {
                instrument_id,
                instrument_stats,
            },
            ServerMessageTagged::SubscribeConfirmation(kind) => {
                ServerMessage::SubscribeConfirmation(kind)
            }
            ServerMessageTagged::UnsubscribeConfirmation(kind) => {
                ServerMessage::UnsubscribeConfirmation(kind)
            }
            ServerMessageTagged::TransactionResult {
                signature,
                response,
            } => ServerMessage::TransactionResult {
                signature,
                response,
            },
            ServerMessageTagged::ExecuteOnDisconnectRegistered {
                signature,
                deferred_nonce,
            } => ServerMessage::ExecuteOnDisconnectRegistered {
                signature,
                deferred_nonce,
            },
            ServerMessageTagged::ExecuteOnDisconnectCleared => {
                ServerMessage::ExecuteOnDisconnectCleared
            }
            ServerMessageTagged::Pong => ServerMessage::Pong,
            ServerMessageTagged::Error(message) => ServerMessage::Error(message),
        }
    }
}

impl<'de> Deserialize<'de> for ServerMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &[
            "QuoteUpdate",
            "OrderbookUpdate",
            "MarkPriceUpdate",
            "AssetMarkPriceUpdate",
            "OrderEventUpdate",
            "CollateralUpdate",
            "PositionUpdate",
            "AccountRiskUpdate",
            "OpenOrdersUpdate",
            "FundingRateUpdate",
            "InstrumentStatsUpdate",
            "CandleUpdate",
            "PositionFundingUpdate",
            "LastMatchPriceUpdate",
            "AuctionFillUpdate",
            "TickerUpdate",
            "SubscribeConfirmation",
            "UnsubscribeConfirmation",
            "TransactionResult",
            "ExecuteOnDisconnectRegistered",
            "ExecuteOnDisconnectCleared",
            "Pong",
            "Error",
        ];

        let value = Value::deserialize(deserializer)?;

        if let Value::Object(map) = &value {
            if map.keys().any(|k| VARIANTS.contains(&k.as_str())) {
                return serde_json::from_value::<ServerMessageTagged>(value)
                    .map(Into::into)
                    .map_err(serde::de::Error::custom);
            }
        }

        #[derive(Deserialize)]
        struct OrderEventUpdateInner {
            #[serde(default)]
            account: Option<AccountAddress>,
            #[serde(rename = "instrumentId", default)]
            instrument_id: Option<InstrumentId>,
            #[serde(rename = "orderEvents")]
            order_events: Vec<OrderEventClientView>,
        }

        let inner: OrderEventUpdateInner =
            serde_json::from_value(value).map_err(serde::de::Error::custom)?;

        Ok(ServerMessage::OrderEventUpdate {
            account: inner.account,
            instrument_id: inner.instrument_id,
            order_events: inner.order_events,
        })
    }
}
