use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, BlockTimestamp};

pub type VaultStatsSetView = Vec<VaultStatsSetViewEntry>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsSetViewEntry {
    pub address: AccountAddress,
    pub stats: VaultStatsView,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsView {
    #[serde(rename = "depositors")]
    pub depositors: usize,
    #[serde(rename = "creationTimestamp")]
    /// Unix timestamp (milliseconds)
    pub creation_timestamp: u64,
    #[serde(rename = "CurrentTVL")]
    pub current_tvl: f64,
    #[serde(rename = "APR")]
    pub apr: f64,
    #[serde(rename = "APRTimeRange")]
    /// Milliseconds
    pub apr_data_time_range: BlockTimestamp,
    #[serde(rename = "APR30d")]
    pub apr_30d: f64,
    /// Milliseconds
    pub apr_30d_data_time_range: BlockTimestamp,
    #[serde(rename = "statsForTimeRange")]
    pub stats_for_time_range: VaultStatsForRangeView,
    pub balances: HashMap<AccountAddress, VaultShareBalanceView>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultShareBalanceView {
    pub shares: u128,
    #[serde(rename = "withdrawableBalance")]
    pub withdrawable_balance: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsForRangeView {
    #[serde(rename = "avgReturns")]
    pub avg_returns: f64,
    #[serde(rename = "returnsVolatility")]
    pub returns_volatility: f64,
    #[serde(rename = "maxDrawdown")]
    pub max_drawdown: f64,
    #[serde(rename = "sharpeRatioDaily")]
    pub sharpe_ratio_daily: f64,
    #[serde(rename = "dailyValues")]
    pub daily_values: Vec<VaultStatsForRangeViewItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsForRangeViewItem {
    /// Milliseconds, first second of the corresponding time period (day)
    pub timestamp: BlockTimestamp,
    pub nav: f64,
    pub tvl: f64,
    #[serde(rename = "return")]
    pub r: f64,
    #[serde(rename = "returnAbsolute")]
    pub r_absolute: f64,
}
