use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, BlockTimestamp};

/// Collection of vault stats entries.
pub type VaultStatsSetView = Vec<VaultStatsSetViewEntry>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsSetViewEntry {
    /// Address of the vault.
    pub address: AccountAddress,
    /// Statistics for the vault.
    pub stats: VaultStatsView,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsView {
    /// Number of depositors in the vault.
    #[serde(rename = "depositors")]
    pub depositors: usize,
    /// Unix timestamp of vault creation (milliseconds).
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: u64,
    /// Current total value locked.
    #[serde(rename = "CurrentTVL")]
    pub current_tvl: f64,
    /// Annual percentage rate.
    #[serde(rename = "APR")]
    pub apr: f64,
    /// Time range of data over which APR was calculated (milliseconds).
    #[serde(rename = "APRTimeRange")]
    pub apr_data_time_range: BlockTimestamp,
    /// 30-day APR.
    #[serde(rename = "APR30d")]
    pub apr_30d: f64,
    /// Time range for 30-day APR data (milliseconds).
    pub apr_30d_data_time_range: BlockTimestamp,
    /// Statistical summary over a custom time range.
    #[serde(rename = "statsForTimeRange")]
    pub stats_for_time_range: VaultStatsForRangeView,
    /// Balances per account share.
    pub balances: HashMap<AccountAddress, VaultShareBalanceView>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultShareBalanceView {
    /// Number of shares held.
    pub shares: u128,
    /// Withdrawable balance (human-readable).
    #[serde(rename = "withdrawableBalance")]
    pub withdrawable_balance: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsForRangeView {
    /// Average returns over the time range.
    #[serde(rename = "avgReturns")]
    pub avg_returns: f64,
    /// Volatility of returns over the time range.
    #[serde(rename = "returnsVolatility")]
    pub returns_volatility: f64,
    /// Maximum drawdown observed.
    #[serde(rename = "maxDrawdown")]
    pub max_drawdown: f64,
    /// Daily Sharpe ratio.
    #[serde(rename = "sharpeRatioDaily")]
    pub sharpe_ratio_daily: f64,
    /// Daily values.
    #[serde(rename = "dailyValues")]
    pub daily_values: Vec<VaultStatsForRangeViewItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultStatsForRangeViewItem {
    /// Milliseconds, first second of the corresponding day.
    pub timestamp: BlockTimestamp,
    /// Notional asset value.
    pub nav: f64,
    /// Total value locked.
    pub tvl: f64,
    /// Return percentage.
    #[serde(rename = "return")]
    pub r: f64,
    /// Absolute return value.
    #[serde(rename = "returnAbsolute")]
    pub r_absolute: f64,
}
