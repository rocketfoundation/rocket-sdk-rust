use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, BlockTimestamp};

/// Ranking metric for the indexer leaderboard.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LeaderboardMetric {
    Pnl,
    Volume,
}

/// Query parameters for the indexer leaderboard.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLeaderboard {
    /// Start timestamp in milliseconds. Omit or pass 0 for all available history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<BlockTimestamp>,
    /// End timestamp in milliseconds. Omit or pass 0 for the latest available data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<BlockTimestamp>,
    /// Maximum number of users to return (optional, defaults to 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    /// Ranking metric. `pnl` orders by portfolio PnL; `volume` orders by fill notional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<LeaderboardMetric>,
    /// Optional account whose leaderboard position should be returned even if outside `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountAddress>,
}

/// Users ranked by the requested metric over the time range.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLeaderboardResponse {
    pub leaderboard: Vec<LeaderboardResponseItem>,
    /// Requested account's 1-based leaderboard position, if `account` was provided and ranked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_position: Option<LeaderboardPositionResponseItem>,
}

/// A ranked leaderboard row.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardResponseItem {
    pub user: AccountAddress,
    pub pnl: String,
    pub volume: String,
}

/// Position of a requested account on the leaderboard.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardPositionResponseItem {
    pub position: u64,
    pub user: AccountAddress,
    pub pnl: String,
    pub volume: String,
}
