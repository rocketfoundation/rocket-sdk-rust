/*
use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionResponse {
    PlaceOrder(PlaceOrderTransactionResponse),
    CreateVault(CreateVaultTransactionResponse),
    VaultDeposit(VaultDepositTransactionResponse),
    VaultWithdraw(VaultWithdrawTransactionResponse),
    DelegateManager(DelegateManagerTransactionResponse),
    UpdateOracleConfig(UpdateOracleConfigResponse),
    ListAssets(ListAssetsResponse),
    ListInstruments(ListInstrumentsResponse),
    Ok,
    Err(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceOrderTransactionResponse {
    pub results: Vec<PlaceOrderResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateVaultTransactionResponse {
    pub vault_address: AccountAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultDepositTransactionResponse {
    pub vault_address: AccountAddress,
    /// String containing a decimal number
    pub deposited_amount: String,
    /// String containing a decimal number
    pub minted_shares: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultWithdrawTransactionResponse {
    pub vault_address: AccountAddress,
    /// String containing a decimal number
    pub burned_shares: String,
    /// String containing a decimal number
    pub payout_quantity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegateManagerTransactionResponse {
    pub delegator: AccountAddress,
    pub manager: AccountAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlaceOrderResult {
    Success(OrderEvent),
    Err(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOracleConfigResponse {
    pub new_quote_symbol_pattern: Option<String>,
    pub updated_oracle_settings: Option<OracleSettingsMap>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAssetsResponse {
    pub assets: Vec<AssetRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInstrumentsResponse {
    pub instruments: Vec<InstrumentRow>,
}
*/
