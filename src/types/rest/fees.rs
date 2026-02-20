use serde::{Deserialize, Serialize};

use crate::types::views::global_fees::GlobalFeesClientView;

/// Params for getting global fee configuration (no params needed).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetGlobalFees;

/// Response returning current global fee configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetGlobalFeesResponse {
    /// Fee configuration payload.
    pub result: GlobalFeesClientView,
}
