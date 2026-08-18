use serde::{Deserialize, Serialize};

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::AccountAddress};

/// A single non-zero vault depositor.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultDepositorView {
    /// Depositor account address.
    pub account: AccountAddress,
    /// Share balance as a decimal string.
    pub shares: String,
    /// Current value of the shares as a decimal string.
    pub value: String,
}

/// Non-zero depositors for a vault.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VaultDepositorsView(Vec<VaultDepositorView>);

impl_as_ref_mut_newtype!(VaultDepositorsView, Vec<VaultDepositorView>);
