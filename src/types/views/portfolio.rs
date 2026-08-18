use serde::{Deserialize, Serialize};

use crate::types::primitives::BlockTimestamp;

/// A point on a portfolio or vault history curve.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioCurvePoint {
    /// Timestamp in milliseconds.
    pub timestamp: BlockTimestamp,
    /// Curve value as a decimal string.
    pub value: String,
}

/// Aggregate portfolio metrics for a requested range.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioSummary {
    pub pnl: String,
    pub total_funding: String,
    pub volume: String,
    pub max_drawdown_pct: String,
    pub return_pct: String,
    pub avg_daily_pnl: String,
    pub pnl_volatility_pct: String,
    pub pnl_volatility_usd: String,
    pub sharpe_ratio: String,
    pub total_trades: u64,
    pub profit_factor: String,
    pub win_rate: String,
    pub wins: String,
    pub losses: String,
    pub average_win: String,
    pub average_loss: String,
    pub long_pnl: String,
    pub short_pnl: String,
}

/// Summary stats derived from vault NAV returns.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultHistoryStats {
    pub sharpe_ratio: String,
    pub average_returns: String,
    pub returns_volatility: String,
    pub max_drawdown: String,
}
