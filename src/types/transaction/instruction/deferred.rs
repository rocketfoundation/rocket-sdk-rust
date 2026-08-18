use serde::{Deserialize, Serialize};

use crate::types::transaction::instruction::order::OrderRequestSet;

/// Pre-signed order wrapper that uses the deferred nonce lane.
///
/// A trusted third party may hold and submit this instruction later. The outer
/// [`crate::types::transaction::RawTransaction::nonce`] is interpreted as
/// the account's deferred nonce (not the ordinary transaction nonce).
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeferredData {
    /// Inclusive expiry in block timestamp milliseconds. Rejected when
    /// `block_timestamp > expires_at_ms`.
    pub expires_at_ms: u64,
    /// Orders to execute if the deferred instruction is still valid.
    pub orders: OrderRequestSet,
}
