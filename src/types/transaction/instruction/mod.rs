use serde::{Deserialize, Serialize};

use crate::types::transaction::instruction::{
    delegate_manager::DelegateManagerData,
    delist_instrument::DelistInstrumentData,
    faucet_claim::FaucetClaimData,
    fee::{SetFeeCollectorData, SetFeeTierData},
    list_asset::{ListAssetsData, UpdateAssetScenariosData},
    list_instrument::ListInstrumentsData,
    mint::MintData,
    oracle::UpdateOracleConfigData,
    order::OrderRequestSet,
    proposal::CreateProposalData,
    set_is_trading::SetIsTradingData,
    set_leverage::SetLeverageData,
    set_main_vault_data::SetMainVaultData,
    vault::{CreateVaultData, VaultDepositData, VaultWithdrawData},
    withdraw::WithdrawData,
};

pub mod delegate_manager;
pub mod delist_instrument;
pub mod faucet_claim;
pub mod fee;
pub mod list_asset;
pub mod list_instrument;
pub mod mint;
pub mod oracle;
pub mod order;
pub mod proposal;
pub mod set_is_trading;
pub mod set_leverage;
pub mod set_main_vault_data;
pub mod vault;
pub mod withdraw;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Mint,
    Burn,
    Withdraw,
    CreateVault,
    VaultDeposit,
    VaultWithdraw,
    DelegateManager,
    CreateProposal,
    VoteOnProposal,
    PlaceOrder,
    ModifyOrder,
    CancelOrder,
    CancelAllOrders,
    SetLeverage,
    UpdateOracleConfig,
    ListAssets,
    ListInstruments,
    SetIsTrading,
    SetFeeTier,
    SetFeeCollector,
    UpdateAssetScenarios,
    FaucetClaim,
    SetMainVault,
    DelistInstrument,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TransactionInstruction {
    Mint(MintData),
    Withdraw(WithdrawData),
    CreateVault(CreateVaultData),
    VaultDeposit(VaultDepositData),
    VaultWithdraw(VaultWithdrawData),
    DelegateManager(DelegateManagerData),
    CreateProposal(CreateProposalData),
    PlaceOrder(OrderRequestSet),
    SetLeverage(SetLeverageData),
    UpdateOracleConfig(UpdateOracleConfigData),
    ListAssets(ListAssetsData),
    ListInstruments(ListInstrumentsData),
    SetIsTrading(SetIsTradingData),
    SetFeeTier(SetFeeTierData),
    SetFeeCollector(SetFeeCollectorData),
    UpdateAssetScenarios(Box<UpdateAssetScenariosData>),
    FaucetClaim(FaucetClaimData),
    SetMainVault(SetMainVaultData),
    DelistInstrument(DelistInstrumentData),
}

impl TransactionInstruction {
    pub fn transaction_type(&self) -> TransactionType {
        match self {
            TransactionInstruction::Mint(_) => TransactionType::Mint,
            TransactionInstruction::Withdraw(_) => TransactionType::Withdraw,
            TransactionInstruction::CreateVault(_) => TransactionType::CreateVault,
            TransactionInstruction::VaultDeposit(_) => TransactionType::VaultDeposit,
            TransactionInstruction::VaultWithdraw(_) => TransactionType::VaultWithdraw,
            TransactionInstruction::DelegateManager(_) => TransactionType::DelegateManager,
            TransactionInstruction::CreateProposal(_) => TransactionType::CreateProposal,
            TransactionInstruction::PlaceOrder(_) => TransactionType::PlaceOrder,
            TransactionInstruction::SetLeverage(_) => TransactionType::SetLeverage,
            TransactionInstruction::UpdateOracleConfig(_) => TransactionType::UpdateOracleConfig,
            TransactionInstruction::ListAssets(_) => TransactionType::ListAssets,
            TransactionInstruction::ListInstruments(_) => TransactionType::ListInstruments,
            TransactionInstruction::SetIsTrading(_) => TransactionType::SetIsTrading,
            TransactionInstruction::SetFeeTier(_) => TransactionType::SetFeeTier,
            TransactionInstruction::SetFeeCollector(_) => TransactionType::SetFeeCollector,
            TransactionInstruction::UpdateAssetScenarios(_) => {
                TransactionType::UpdateAssetScenarios
            }
            TransactionInstruction::FaucetClaim(_) => TransactionType::FaucetClaim,
            TransactionInstruction::SetMainVault(_) => TransactionType::SetMainVault,
            TransactionInstruction::DelistInstrument(_) => TransactionType::DelistInstrument,
        }
    }
}
