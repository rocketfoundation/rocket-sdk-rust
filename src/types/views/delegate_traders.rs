use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Delegate trader attached to an account or vault.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateTraderView {
    /// Delegate address.
    pub address: AccountAddress,
    /// Optional expiry timestamp in milliseconds.
    pub expiry_ms: Option<u64>,
    /// Optional display name.
    pub name: Option<String>,
    /// Whether the delegate is a web-client session key.
    pub is_web_client: bool,
}
