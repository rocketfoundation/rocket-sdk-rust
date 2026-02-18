use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::AssetTicker;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UpdateOracleConfigPriceScale {
    pub numerator: String,
    pub denominator: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOracleConfigState {
    #[serde(rename = "assetTickerOverrides")]
    pub asset_ticker_overrides: Option<HashMap<AssetTicker, AssetTicker>>,
    #[serde(rename = "priceScales")]
    pub price_scales: Option<HashMap<String, UpdateOracleConfigPriceScale>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UpdateOracleConfigData {
    #[serde(rename = "quoteSymbolPattern")]
    pub quote_symbol_pattern: Option<String>,
    #[serde(rename = "oracleSettings")]
    pub oracle_settings: Option<HashMap<String, UpdateOracleConfigState>>,
}
