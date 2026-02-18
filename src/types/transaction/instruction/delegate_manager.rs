use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegateManagerData {
    pub manager: AccountAddress,
    /// Timestamp milliseconds
    pub expiry_ms: Option<String>,
}
