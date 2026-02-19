use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Payload for delegating management rights to another account.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegateManagerData {
    /// Address of the manager.
    pub manager: AccountAddress,
    /// Optional manager role expiration timestamp in milliseconds.
    pub expiry_ms: Option<String>,
}
