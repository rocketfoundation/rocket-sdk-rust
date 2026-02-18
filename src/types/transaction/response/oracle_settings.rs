use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::AssetTicker;

pub type OracleSettingsMap = HashMap<SourceId, OracleState>;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum SourceId {
    Mock,
    Deribit,
    BinanceSpot,
    MEXC,
    Pyth,
    Hyperliquid,
    CommoditiesAPI,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct OracleState {
    /// Override asset ticker for a given source.
    /// Example: ( "BTC": "XBC" ) - replaces btc with xbc for deribit source.
    pub asset_ticker_overrides: HashMap<AssetTicker, AssetTicker>,
    /// Apply price scales for specific tickers.
    /// If set, price from source is multiplied by numerator and divided by denominator.
    #[serde(default)]
    pub price_scales: HashMap<AssetTicker, OraclePriceScale>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct OraclePriceScale {
    pub numerator: u64,
    pub denominator: u64,
}
