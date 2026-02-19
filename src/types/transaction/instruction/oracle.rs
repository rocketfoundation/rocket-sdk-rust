use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::AssetTicker;

/// Price scaling parameters for oracle configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UpdateOracleConfigPriceScale {
    /// Numerator of the scale as a decimal string.
    pub numerator: String,
    /// Denominator of the scale as a decimal string.
    pub denominator: String,
}

/// State updates for an oracle source.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOracleConfigState {
    /// Optional mapping to override asset tickers.
    #[serde(rename = "assetTickerOverrides")]
    pub asset_ticker_overrides: Option<HashMap<AssetTicker, AssetTicker>>,
    /// Optional mapping of price scales keyed by asset ticker.
    #[serde(rename = "priceScales")]
    pub price_scales: Option<HashMap<String, UpdateOracleConfigPriceScale>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
/// Top-level payload for updating oracle configuration options.
pub struct UpdateOracleConfigData {
    /// Optional regex pattern to recognize valid quote symbols.
    #[serde(rename = "quoteSymbolPattern")]
    pub quote_symbol_pattern: Option<String>,
    /// Optional oracle settings keyed by source id.
    #[serde(rename = "oracleSettings")]
    pub oracle_settings: Option<HashMap<String, UpdateOracleConfigState>>,
}
