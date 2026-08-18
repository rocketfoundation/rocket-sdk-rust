use serde::{Deserialize, Serialize};

use crate::types::primitives::BlockTimestamp;

/// Mark price or implied volatility update.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MarkPriceView {
    Price {
        price: String,
        timestamp: BlockTimestamp,
    },
    PriceIV {
        price: String,
        iv: String,
        /// Black-Scholes delta. `None` until Greeks are available.
        #[serde(default)]
        delta: Option<String>,
        /// Black-Scholes gamma. `None` until Greeks are available.
        #[serde(default)]
        gamma: Option<String>,
        /// Black-Scholes theta (per calendar day). `None` until Greeks are available.
        #[serde(default)]
        theta: Option<String>,
        /// Black-Scholes vega (per 1 vol point). `None` until Greeks are available.
        #[serde(default)]
        vega: Option<String>,
        /// Black-Scholes rho (per 1 percentage point of rate). `None` until Greeks are available.
        #[serde(default)]
        rho: Option<String>,
        timestamp: BlockTimestamp,
    },
}
