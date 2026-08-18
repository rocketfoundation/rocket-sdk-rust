use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

/// Payload for delegating management rights to another account.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DelegateManagerData {
    /// Address of the manager.
    pub manager: AccountAddress,
    /// Optional manager role expiration timestamp in milliseconds.
    pub expiry_ms: Option<u64>,
    /// Optional display name for the delegate (max 20 characters).
    pub name: Option<String>,
    /// When true, the delegate is treated as a web-client session key.
    pub is_web_client: Option<bool>,
}

/// Payload for removing a previously granted manager.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDelegateManagerData {
    /// Address of the manager to remove.
    pub manager: AccountAddress,
}

/// Payload for clearing every web-client delegate on the sender account.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RemoveWebclientDelegatesData {}
