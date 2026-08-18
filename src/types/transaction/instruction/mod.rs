use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress,
    transaction::instruction::{
        deferred::DeferredData,
        delegate_manager::{
            DelegateManagerData, RemoveDelegateManagerData, RemoveWebclientDelegatesData,
        },
        delist_instrument::DelistInstrumentData,
        faucet_claim::FaucetClaimData,
        fee::{SetFeeCollectorData, SetFeeTierData},
        list_asset::{ListAssetsData, UpdateAssetScenariosData},
        list_instrument::ListInstrumentsData,
        mint::MintData,
        oracle::UpdateOracleConfigData,
        order::{ModifyTWAPRequest, OrderRequest, OrderRequestSet, PlaceTWAPRequest},
        proposal::CreateProposalData,
        set_is_trading::SetIsTradingData,
        set_leverage::SetLeverageData,
        set_main_vault_data::SetMainVaultData,
        set_market_maker_protection::SetMarketMakerProtectionData,
        set_max_funding_rate::SetMaxFundingRateData,
        vault::{CreateVaultData, VaultDepositData, VaultWithdrawData},
        withdraw::WithdrawData,
    },
};

pub mod deferred;
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
pub mod set_market_maker_protection;
pub mod set_max_funding_rate;
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
    SetMaxFundingRate,
    RemoveDelegateManager,
    PlaceTWAP,
    ModifyTWAP,
    RemoveWebclientDelegates,
    SetMarketMakerProtection,
    Deferred,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TransactionInstruction {
    // NOTE: Variant order matters for MessagePack serialization compatibility with stored blocks.
    // New variants MUST be appended at the end. Never insert in the middle.
    Mint(MintData),                                         // 0
    Withdraw(WithdrawData),                                 // 1
    CreateVault(CreateVaultData),                           // 2
    VaultDeposit(VaultDepositData),                         // 3
    VaultWithdraw(VaultWithdrawData),                       // 4
    DelegateManager(DelegateManagerData),                   // 5
    CreateProposal(CreateProposalData),                     // 6
    PlaceOrder(OrderRequestSet),                            // 7
    SetLeverage(SetLeverageData),                           // 8
    UpdateOracleConfig(UpdateOracleConfigData),             // 9
    ListAssets(ListAssetsData),                             // 10
    ListInstruments(ListInstrumentsData),                   // 11
    SetIsTrading(SetIsTradingData),                         // 12
    SetFeeTier(SetFeeTierData),                             // 13
    SetFeeCollector(SetFeeCollectorData),                   // 14
    UpdateAssetScenarios(Box<UpdateAssetScenariosData>),    // 15
    FaucetClaim(FaucetClaimData),                           // 16
    SetMainVault(SetMainVaultData),                         // 17
    DelistInstrument(DelistInstrumentData),                 // 18
    SetMaxFundingRate(SetMaxFundingRateData),               // 19
    RemoveDelegateManager(RemoveDelegateManagerData),       // 20
    PlaceTWAP(PlaceTWAPRequest),                            // 21
    ModifyTWAP(ModifyTWAPRequest),                          // 22
    RemoveWebclientDelegates(RemoveWebclientDelegatesData), // 23
    SetMarketMakerProtection(SetMarketMakerProtectionData), // 24
    Deferred(DeferredData),                                 // 25
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
            TransactionInstruction::RemoveDelegateManager(_) => {
                TransactionType::RemoveDelegateManager
            }
            TransactionInstruction::CreateProposal(_) => TransactionType::CreateProposal,
            TransactionInstruction::PlaceOrder(_) => TransactionType::PlaceOrder,
            TransactionInstruction::PlaceTWAP(_) => TransactionType::PlaceTWAP,
            TransactionInstruction::ModifyTWAP(_) => TransactionType::ModifyTWAP,
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
            TransactionInstruction::SetMaxFundingRate(_) => TransactionType::SetMaxFundingRate,
            TransactionInstruction::RemoveWebclientDelegates(_) => {
                TransactionType::RemoveWebclientDelegates
            }
            TransactionInstruction::SetMarketMakerProtection(_) => {
                TransactionType::SetMarketMakerProtection
            }
            TransactionInstruction::Deferred(_) => TransactionType::Deferred,
        }
    }

    /// True when this instruction uses the deferred nonce lane.
    pub fn uses_deferred_nonce(&self) -> bool {
        matches!(self, TransactionInstruction::Deferred(_))
    }

    /// Account(s) whose state this instruction mutates. Order instructions carry
    /// an explicit `trader` distinct from the signing `sender` when a delegate
    /// manager submits on behalf of a vault.
    pub fn traders(&self) -> Vec<&AccountAddress> {
        match self {
            TransactionInstruction::PlaceOrder(orders)
            | TransactionInstruction::Deferred(DeferredData { orders, .. }) => {
                orders.iter().map(OrderRequest::trader).collect()
            }
            TransactionInstruction::PlaceTWAP(request) => vec![&request.trader],
            TransactionInstruction::ModifyTWAP(request) => vec![&request.trader],
            _ => Vec::new(),
        }
    }
}
