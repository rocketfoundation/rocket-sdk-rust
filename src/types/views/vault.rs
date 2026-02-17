use serde::{Deserialize, Serialize};

use crate::{
    impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, AssetId, BlockTimestamp},
};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct VaultSetView(Vec<VaultView>);

impl_as_ref_mut_newtype!(VaultSetView, Vec<VaultView>);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VaultView {
    pub address: AccountAddress,
    pub manager: AccountAddress,
    pub asset: AssetId,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: BlockTimestamp,
}
