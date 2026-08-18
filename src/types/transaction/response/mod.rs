mod oracle_settings;
mod order;

pub use oracle_settings::*;
pub use order::*;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::types::primitives::{AccountAddress, AssetRow, InstrumentRow};

/// General response wrapper for different transaction types.
#[derive(Serialize, Debug, Clone)]
pub enum TransactionResponse {
    /// Response for a place order transaction.
    PlaceOrder(PlaceOrderTransactionResponse),
    /// Response after creating a vault.
    CreateVault(CreateVaultTransactionResponse),
    /// Response after depositing into a vault.
    VaultDeposit(VaultDepositTransactionResponse),
    /// Response after withdrawing from a vault.
    VaultWithdraw(VaultWithdrawTransactionResponse),
    /// Response for a delegate manager transaction.
    DelegateManager(DelegateManagerTransactionResponse),
    /// Response for removing a delegate manager.
    RemoveDelegateManager(RemoveDelegateManagerTransactionResponse),
    /// Response for clearing web-client delegates.
    RemoveWebclientDelegates(RemoveWebclientDelegatesTransactionResponse),
    /// Response to oracle configuration update.
    UpdateOracleConfig(UpdateOracleConfigResponse),
    /// List assets response.
    ListAssets(ListAssetsResponse),
    /// List instruments response.
    ListInstruments(ListInstrumentsResponse),
    /// Error response with a message.
    Err(String),
    /// Generic ok response with no payload.
    Ok,
}

/// Result details for a place order transaction.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceOrderTransactionResponse {
    /// List of individual order results.
    pub results: Vec<PlaceOrderResult>,
}

/// Response payload for vault creation transactions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateVaultTransactionResponse {
    /// Address of the newly created vault.
    pub vault_address: AccountAddress,
}

/// Response for depositing into a vault.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultDepositTransactionResponse {
    /// Address of the vault that received the deposit.
    pub vault_address: AccountAddress,
    /// Amount deposited as a decimal string.
    pub deposited_amount: String,
    /// Number of shares minted as a decimal string.
    pub minted_shares: String,
}

/// Response for withdrawing from a vault.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultWithdrawTransactionResponse {
    /// Address of the vault from which funds were withdrawn.
    pub vault_address: AccountAddress,
    /// Number of shares burned as a decimal string.
    pub burned_shares: String,
    /// Payout quantity as a decimal string.
    pub payout_quantity: String,
}

/// Response for delegate manager updates.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegateManagerTransactionResponse {
    /// Address of the delegator.
    pub delegator: AccountAddress,
    /// Address of the new manager.
    pub manager: AccountAddress,
}

/// Response for removing a delegate manager.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveDelegateManagerTransactionResponse {
    /// Address of the delegator.
    pub delegator: AccountAddress,
    /// Address of the removed manager.
    pub manager: AccountAddress,
}

/// Response for clearing web-client delegates.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveWebclientDelegatesTransactionResponse {
    /// Number of removed web-client delegates.
    pub removed_count: u64,
}

/// Response for oracle configuration update. Contains the applied changes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOracleConfigResponse {
    pub new_quote_symbol_pattern: Option<String>,
    pub updated_oracle_settings: Option<OracleSettingsMap>,
}

/// Response containing the listed assets data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAssetsResponse {
    pub assets: Vec<AssetRow>,
}

/// Response containing the listed instruments data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInstrumentsResponse {
    pub instruments: Vec<InstrumentRow>,
}

impl<'de> Deserialize<'de> for TransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = match Value::deserialize(deserializer) {
            Ok(value) => value,
            Err(err) => {
                // Empty response body corresponds to Ok.
                if err.to_string().starts_with("EOF") {
                    return Ok(TransactionResponse::Ok);
                } else {
                    return Err(err);
                }
            }
        };

        // Empty payload means Ok.
        if value.is_null() {
            return Ok(TransactionResponse::Ok);
        }

        // If response is a string, it's an error.
        if let Value::String(s) = &value {
            return Ok(TransactionResponse::Err(s.clone()));
        }

        // Must be an object from here on.
        let obj = value.as_object().ok_or_else(|| {
            serde::de::Error::custom("Expected JSON object for TransactionResponse")
        })?;

        // If it's an empty object, treat as Ok.
        if obj.is_empty() {
            return Ok(TransactionResponse::Ok);
        }

        // WS `TransactionResult` serializes the tagged enum form.
        const TAGGED: &[&str] = &[
            "PlaceOrder",
            "CreateVault",
            "VaultDeposit",
            "VaultWithdraw",
            "DelegateManager",
            "RemoveDelegateManager",
            "RemoveWebclientDelegates",
            "UpdateOracleConfig",
            "ListAssets",
            "ListInstruments",
            "Ok",
            "Err",
        ];
        if obj.len() == 1 {
            if let Some(key) = obj.keys().next() {
                if TAGGED.contains(&key.as_str()) {
                    #[derive(Deserialize)]
                    enum Tagged {
                        PlaceOrder(PlaceOrderTransactionResponse),
                        CreateVault(CreateVaultTransactionResponse),
                        VaultDeposit(VaultDepositTransactionResponse),
                        VaultWithdraw(VaultWithdrawTransactionResponse),
                        DelegateManager(DelegateManagerTransactionResponse),
                        RemoveDelegateManager(RemoveDelegateManagerTransactionResponse),
                        RemoveWebclientDelegates(RemoveWebclientDelegatesTransactionResponse),
                        UpdateOracleConfig(UpdateOracleConfigResponse),
                        ListAssets(ListAssetsResponse),
                        ListInstruments(ListInstrumentsResponse),
                        Err(String),
                        Ok,
                    }

                    return serde_json::from_value::<Tagged>(value)
                        .map(|tagged| match tagged {
                            Tagged::PlaceOrder(v) => TransactionResponse::PlaceOrder(v),
                            Tagged::CreateVault(v) => TransactionResponse::CreateVault(v),
                            Tagged::VaultDeposit(v) => TransactionResponse::VaultDeposit(v),
                            Tagged::VaultWithdraw(v) => TransactionResponse::VaultWithdraw(v),
                            Tagged::DelegateManager(v) => TransactionResponse::DelegateManager(v),
                            Tagged::RemoveDelegateManager(v) => {
                                TransactionResponse::RemoveDelegateManager(v)
                            }
                            Tagged::RemoveWebclientDelegates(v) => {
                                TransactionResponse::RemoveWebclientDelegates(v)
                            }
                            Tagged::UpdateOracleConfig(v) => {
                                TransactionResponse::UpdateOracleConfig(v)
                            }
                            Tagged::ListAssets(v) => TransactionResponse::ListAssets(v),
                            Tagged::ListInstruments(v) => TransactionResponse::ListInstruments(v),
                            Tagged::Err(v) => TransactionResponse::Err(v),
                            Tagged::Ok => TransactionResponse::Ok,
                        })
                        .map_err(serde::de::Error::custom);
                }
            }
        }

        if obj.contains_key("results") {
            return serde_json::from_value::<PlaceOrderTransactionResponse>(value)
                .map(TransactionResponse::PlaceOrder)
                .map_err(serde::de::Error::custom);
        }

        if obj.contains_key("vault_address") {
            if obj.contains_key("deposited_amount") {
                return serde_json::from_value::<VaultDepositTransactionResponse>(value)
                    .map(TransactionResponse::VaultDeposit)
                    .map_err(serde::de::Error::custom);
            } else if obj.contains_key("payout_quantity") {
                return serde_json::from_value::<VaultWithdrawTransactionResponse>(value)
                    .map(TransactionResponse::VaultWithdraw)
                    .map_err(serde::de::Error::custom);
            } else {
                return serde_json::from_value::<CreateVaultTransactionResponse>(value)
                    .map(TransactionResponse::CreateVault)
                    .map_err(serde::de::Error::custom);
            }
        }

        if obj.contains_key("removed_count") {
            return serde_json::from_value::<RemoveWebclientDelegatesTransactionResponse>(value)
                .map(TransactionResponse::RemoveWebclientDelegates)
                .map_err(serde::de::Error::custom);
        }

        if obj.contains_key("delegator") {
            return serde_json::from_value::<DelegateManagerTransactionResponse>(value)
                .map(TransactionResponse::DelegateManager)
                .map_err(serde::de::Error::custom);
        }

        if obj.contains_key("assets") {
            return serde_json::from_value::<ListAssetsResponse>(value)
                .map(TransactionResponse::ListAssets)
                .map_err(serde::de::Error::custom);
        }

        if obj.contains_key("instruments") {
            return serde_json::from_value::<ListInstrumentsResponse>(value)
                .map(TransactionResponse::ListInstruments)
                .map_err(serde::de::Error::custom);
        }

        if obj.contains_key("new_quote_symbol_pattern")
            || obj.contains_key("updated_oracle_settings")
        {
            return serde_json::from_value::<UpdateOracleConfigResponse>(value)
                .map(TransactionResponse::UpdateOracleConfig)
                .map_err(serde::de::Error::custom);
        }

        Err(serde::de::Error::custom(
            "Unknown transaction response variant",
        ))
    }
}
