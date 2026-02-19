mod oracle_settings;
mod order;

pub use oracle_settings::*;
pub use order::*;

use serde::{Deserialize, Serialize};

use crate::types::primitives::{AccountAddress, AssetRow, InstrumentRow};

/// General response wrapper for different transaction types.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    /// Response to oracle configuration update.
    UpdateOracleConfig(UpdateOracleConfigResponse),
    /// List assets response.
    ListAssets(ListAssetsResponse),
    /// List instruments response.
    ListInstruments(ListInstrumentsResponse),
    /// Generic ok response with no payload.
    Ok,
    /// Error response with a message.
    Err(String),
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
