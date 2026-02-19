use serde::{Deserialize, Serialize};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, AssetId, BlockTimestamp},
};

/// Collection of vault views.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct VaultSetView(Vec<VaultView>);

impl_as_ref_mut_newtype!(VaultSetView, Vec<VaultView>);

/// Vault data.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VaultView {
    /// Address of the vault.
    pub address: AccountAddress,
    /// Manager's account address.
    pub manager: AccountAddress,
    /// Asset identifier.
    pub asset: AssetId,
    /// Creation timestamp of the vault.
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: BlockTimestamp,
}
