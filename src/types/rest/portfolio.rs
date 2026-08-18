use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::{AccountAddress, BlockTimestamp, CandleTimeframe},
    views::portfolio::{PortfolioCurvePoint, PortfolioSummary},
};

/// Query parameters for an account's portfolio curves and summary.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPortfolio {
    pub account: AccountAddress,
    /// Start timestamp in milliseconds (inclusive). Use 0 for all available history.
    pub from: BlockTimestamp,
    /// End timestamp in milliseconds (inclusive). Use 0 for the latest available snapshot.
    pub to: BlockTimestamp,
    /// Aggregation interval for the portfolio curves.
    pub interval: CandleTimeframe,
}

/// Equity / PnL curves and aggregate metrics for the requested range.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPortfolioResponse {
    /// Equity curve sampled from account snapshots.
    pub equity: Vec<PortfolioCurvePoint>,
    /// PnL curve derived from equity, net of bridge and vault cash flows.
    pub pnl: Vec<PortfolioCurvePoint>,
    /// Aggregate portfolio metrics for the requested range.
    pub summary: PortfolioSummary,
}
