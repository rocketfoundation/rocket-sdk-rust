use serde::{Deserialize, Serialize};

use crate::types::ws::subscription_kind::SubscriptionKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Subscribe(SubscriptionKind),
    Unsubscribe(SubscriptionKind),
    Ping,
}
