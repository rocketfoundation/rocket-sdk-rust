use serde::{Deserialize, Serialize};

use crate::types::{transaction::Transaction, ws::subscription_kind::SubscriptionKind};

/// Messages sent by clients over the WebSocket connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Subscribe request. Does not cancel previous subscriptions.
    Subscribe(SubscriptionKind),
    /// Unsubscribe request. Contents must exactly match the ones passed in the subscription request.
    Unsubscribe(SubscriptionKind),
    /// Ping message to keep the connection alive.
    Ping,
    /// Submit a signed transaction for immediate execution (same semantics as REST).
    SubmitTransaction(Transaction),
    /// Store a signed `Deferred` transaction and submit it when this WebSocket disconnects.
    RegisterExecuteOnDisconnect(Transaction),
    /// Clear any execute-on-disconnect registration without submitting it.
    ClearExecuteOnDisconnect,
}
